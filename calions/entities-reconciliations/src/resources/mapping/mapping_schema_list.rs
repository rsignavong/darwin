use super::MappingSchema;

pub struct MappingSchemaList(Vec<MappingSchema>);

impl MappingSchemaList {
    pub fn add(&mut self, schema: MappingSchema) {
        if !self.0.contains(&schema) {
            self.0.push(schema);
        }
    }

    pub fn for_each<F>(&self, mut func: F)
    where
        F: FnMut(&MappingSchema),
    {
        for schema in self.0.iter() {
            func(schema);
        }
    }

    pub fn new(schema: MappingSchema) -> Self {
        MappingSchemaList(vec![schema])
    }
}
