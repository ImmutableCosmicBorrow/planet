use super::Ai;
use super::decide::{generate_basic_resource, generate_complex_resource};
use common_game::components::planet::PlanetState;
use common_game::components::resource::{
    BasicResource, BasicResourceType, Combinator, ComplexResource, ComplexResourceRequest,
    Generator,
};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};

pub(super) fn handle_message(
    ai: &mut Ai,
    state: &mut PlanetState,
    generator: &Generator,
    combinator: &Combinator,
    msg: ExplorerToPlanet,
) -> Option<PlanetToExplorer> {
    if ai.is_ai_active {
        match msg {
            ExplorerToPlanet::SupportedResourceRequest { .. } => {
                Some(supported_resources(generator))
            }

            ExplorerToPlanet::SupportedCombinationRequest { .. } => {
                Some(supported_combinations(combinator))
            }
            ExplorerToPlanet::GenerateResourceRequest { resource, .. } => {
                Some(generate_resource(ai, state, generator, resource))
            }

            ExplorerToPlanet::CombineResourceRequest { msg, .. } => {
                Some(combine_resource(ai, state, combinator, msg))
            }

            ExplorerToPlanet::AvailableEnergyCellRequest { .. } => {
                Some(PlanetToExplorer::AvailableEnergyCellResponse {
                    available_cells: u32::from(state.cell(0).is_charged()),
                })
            }
        }
    } else {
        None
    }
}

/// Returns the available Basic Resources set of the planet
fn supported_resources(generator: &Generator) -> PlanetToExplorer {
    PlanetToExplorer::SupportedResourceResponse {
        resource_list: generator.all_available_recipes(),
    }
}

/// Returns the available Complex Resources set of the planet
fn supported_combinations(combinator: &Combinator) -> PlanetToExplorer {
    PlanetToExplorer::SupportedCombinationResponse {
        combination_list: combinator.all_available_recipes(),
    }
}

/// Return the optional Basic resource generated
fn generate_resource(
    ai: &mut Ai,
    state: &mut PlanetState,
    generator: &Generator,
    to_generate: BasicResourceType,
) -> PlanetToExplorer {
    if !generate_basic_resource(ai, state) {
        return PlanetToExplorer::GenerateResourceResponse { resource: None };
    }

    let resource = match to_generate {
        BasicResourceType::Carbon => generator
            .make_carbon(state.cell_mut(0))
            .ok()
            .map(BasicResource::Carbon),
        BasicResourceType::Hydrogen => generator
            .make_hydrogen(state.cell_mut(0))
            .ok()
            .map(BasicResource::Hydrogen),
        BasicResourceType::Silicon => generator
            .make_silicon(state.cell_mut(0))
            .ok()
            .map(BasicResource::Silicon),
        BasicResourceType::Oxygen => generator
            .make_oxygen(state.cell_mut(0))
            .ok()
            .map(BasicResource::Oxygen),
    };

    PlanetToExplorer::GenerateResourceResponse { resource }
}

/// Returns the optional complex resource created
fn combine_resource(
    ai: &mut Ai,
    state: &mut PlanetState,
    combinator: &Combinator,
    msg: ComplexResourceRequest,
) -> PlanetToExplorer {
    if !generate_complex_resource(ai, state) {
        let response = match msg {
            ComplexResourceRequest::Water(r1, r2) => Err((
                "Keeping the energy cell".to_string(),
                r1.to_generic(),
                r2.to_generic(),
            )),

            ComplexResourceRequest::Diamond(r1, r2) => Err((
                "Keeping the energy cell".to_string(),
                r1.to_generic(),
                r2.to_generic(),
            )),

            ComplexResourceRequest::Life(r1, r2) => Err((
                "Keeping the energy cell".to_string(),
                r1.to_generic(),
                r2.to_generic(),
            )),

            ComplexResourceRequest::Robot(r1, r2) => Err((
                "Keeping the energy cell".to_string(),
                r1.to_generic(),
                r2.to_generic(),
            )),

            ComplexResourceRequest::Dolphin(r1, r2) => Err((
                "Keeping the energy cell".to_string(),
                r1.to_generic(),
                r2.to_generic(),
            )),

            ComplexResourceRequest::AIPartner(r1, r2) => Err((
                "Keeping the energy cell".to_string(),
                r1.to_generic(),
                r2.to_generic(),
            )),
        };

        return PlanetToExplorer::CombineResourceResponse {
            complex_response: response,
        };
    }

    //trying to craft resource
    let complex_response = match msg {
        ComplexResourceRequest::Water(r1, r2) => combinator
            .make_water(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Water)
            .map_err(|(s, r1, r2)| (s, r1.to_generic(), r2.to_generic())),

        ComplexResourceRequest::Diamond(r1, r2) => combinator
            .make_diamond(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Diamond)
            .map_err(|(s, r1, r2)| (s, r1.to_generic(), r2.to_generic())),

        ComplexResourceRequest::Life(r1, r2) => combinator
            .make_life(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Life)
            .map_err(|(s, r1, r2)| (s, r1.to_generic(), r2.to_generic())),

        ComplexResourceRequest::Robot(r1, r2) => combinator
            .make_robot(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Robot)
            .map_err(|(s, r1, r2)| (s, r1.to_generic(), r2.to_generic())),

        ComplexResourceRequest::Dolphin(r1, r2) => combinator
            .make_dolphin(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Dolphin)
            .map_err(|(s, r1, r2)| (s, r1.to_generic(), r2.to_generic())),

        ComplexResourceRequest::AIPartner(r1, r2) => combinator
            .make_aipartner(r1, r2, state.cell_mut(0))
            .map(ComplexResource::AIPartner)
            .map_err(|(s, r1, r2)| (s, r1.to_generic(), r2.to_generic())),
    };

    PlanetToExplorer::CombineResourceResponse { complex_response }
}
