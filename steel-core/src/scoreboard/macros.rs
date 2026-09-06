macro_rules! define_custom_vanilla_criteria {
    (
        $(
            $(#[$attr:meta])*
            $static_name: ident ( $name: literal ) $(=> $read_only: literal, $render_type: expr)?
        ),* $(,)?
    ) => {
        $(
            vanilla_criterion! {
                $(#[$attr])*
                $static_name ( $name ) $(=> $read_only, $render_type)?
            }
        )*

        fn register_vanilla_custom_criteria(criteria: &mut ObjectiveCriteria) {
            $(
                criteria.vanilla_custom_criteria.insert($name, &$static_name);
            )*
        }
    };
}

macro_rules! define_team_vanilla_criteria {
    (
        $(
            $(#[$attr:meta])*
            $static_name: ident ( $prefix: literal )
        ),* $(,)?
    ) => {
        $(
            $(#[$attr])*
            pub static $static_name: StaticTeamObjectiveCriteria = StaticTeamObjectiveCriteria::new($prefix);
        )*

        fn register_team_criteria(criteria: &mut ObjectiveCriteria) {
            $(
                $static_name.register(criteria);
            )*
        }
    };
}

macro_rules! vanilla_criterion {
    (
        $(#[$attr:meta])*
        $static_name: ident ( $name: literal )
    ) => {
        $(#[$attr])*
        pub static $static_name: StaticObjectiveCriterion = StaticObjectiveCriterion::custom($name);
    };

    (
        $(#[$attr:meta])*
        $static_name: ident ( $name: literal ) => $read_only: literal, $render_type: expr
    ) => {
        $(#[$attr])*
        pub static $static_name: StaticObjectiveCriterion = StaticObjectiveCriterion::custom_with_properties($name, $read_only, $render_type);
    };
}

pub(super) use define_custom_vanilla_criteria;
pub(super) use define_team_vanilla_criteria;
pub(super) use vanilla_criterion;
