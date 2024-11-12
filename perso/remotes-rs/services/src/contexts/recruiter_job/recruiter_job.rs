use super::RecruiterJobError;
use repository::PgConn;
use resources::commands::RecruiterJobCommand;
use resources::entities::JobDetailVersion;
use resources::entities::UserId;
use resources::queries::RecruiterJobEdited;
use std::convert::TryInto;

pub struct RecruiterJob;

impl RecruiterJob {
    pub fn create(
        conn: &PgConn,
        user_id: Option<UserId>,
        recruiter_job_command: RecruiterJobCommand,
    ) -> Result<RecruiterJobEdited, RecruiterJobError> {
        let job = conn.transaction::<RecruiterJobEdited, RecruiterJobError, _>(|| {
            let user_id = match user_id {
                Some(user_id) => Some(*user_id),
                None => {
                    recruiter_job_command
                        .user_email
                        .as_ref()
                        .and_then(|user_email| {
                            // to refactor as a service
                            let email = user_email.to_string();
                            let user = User::find_by_email(conn, &email).ok();
                            match user {
                                Some(user) => Some(user.id),
                                None => {
                                    let user_command = UserCommand::new(email, None);
                                    let user = User::create(conn, &user_command).ok()?;
                                    Some(user.id)
                                }
                            }
                        })
                }
            }
            .ok_or_else(|| RecruiterJobError::CreateUnknownUser)?;

            let job_command = JobCommand::new(
                *recruiter_job_command.category_id,
                *recruiter_job_command.type_id,
                None,
            );
            let job = Job::create(conn, &job_command)?;

            let job_detail_command = JobDetailCommand::new(
                job.id,
                user_id,
                recruiter_job_command.position.to_string(),
                recruiter_job_command.description.to_string(),
                recruiter_job_command.apply.to_string(),
                recruiter_job_command.apply_email.to_string(),
                recruiter_job_command.apply_url_to_string(),
                recruiter_job_command.location_to_string(),
                recruiter_job_command.salary_to_string(),
                *JobDetailVersion::new(),
                JobDetailStatus::Draft,
            );
            let job_detail = JobDetail::create(conn, &job_detail_command)?;

            let category = JobCategory::read(conn, &*recruiter_job_command.category_id)?;
            let type_ = JobType::read(conn, &*recruiter_job_command.type_id)?;
            let recruiter_job_edited = RecruiterJobEdited::new(
                job.into(),
                job_detail.try_into()?,
                category.into(),
                type_.into(),
            );

            Ok(recruiter_job_edited)
        })?;

        Ok(job)
    }
}
