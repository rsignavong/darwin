CREATE
OR REPLACE FUNCTION create_advertiser_post(
  _user_id UUID DEFAULT NULL,
  _user_email VARCHAR(254) DEFAULT NULL,
  _details JSON DEFAULT '{}' :: json
) RETURN BOOLEAN LANGUAGE PLPGSQL AS $$
DECLARE
  user_id UUID;
  post_id UUID;
  result BOOLEAN := FALSE;
BEGIN
  IF _user_id IS NOT NULL THEN 
    user_id := _user_id
  ELSIF _user_email IS NOT NULL THEN
    INSERT INTO users (email)
    VALUES (_user_email)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_id;
  ELSE
    RAISE EXCEPTION 'Missing user id or email', NOW();
  END IF;

  INSERT INTO posts (company_id)
  VALUES (NULL)
  RETURNING id INTO post_id;

  INSERT INTO post_details (post_id, user_id, details)
  VALUES (post_id, user_id, _details);

  IF user_id IS NOT NULL THEN 
    result := TRUE;
  END IF;

  RETURN result;
END;
$$
