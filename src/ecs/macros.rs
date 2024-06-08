
macro_rules! impl_query {
        ($($name:ident),*) => {
            pub fn query<$($name),*>(
                &self,
                entity: Entity,
            ) -> Option<($(Ref<'_, $name>,)*)>
            where
                $($name: 'static + Component),*
            {
                Some((
                    $(Ref::map(self.get_component::<$name>()?, |entity_map| {
                        entity_map.get(entity).unwrap()
                    }),)*
                ))
            }
        }
    }
