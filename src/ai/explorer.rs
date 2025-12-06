use super::Ai;
use common_game::components::planet::PlanetState;
use common_game::components::resource::{
    BasicResource, BasicResourceType, Combinator, ComplexResource, ComplexResourceRequest,
    ComplexResourceType, Generator, GenericResource,
};
use common_game::protocols::messages::{ExplorerToPlanet, PlanetToExplorer};
use std::collections::HashSet;

pub(super) fn handle_message(
    ai: &mut Ai,
    state: &mut PlanetState,
    generator: &Generator,
    combinator: &Combinator,
    msg: ExplorerToPlanet,
) -> Option<PlanetToExplorer> {
    match msg {
        ExplorerToPlanet::SupportedResourceRequest { .. } => {
            Some(PlanetToExplorer::SupportedResourceResponse {
                resource_list: supported_resources(generator),
            })
        }

        ExplorerToPlanet::SupportedCombinationRequest { .. } => {
            Some(PlanetToExplorer::SupportedCombinationResponse {
                combination_list: supported_combinations(combinator),
            })
        }

        ExplorerToPlanet::GenerateResourceRequest { resource, .. } => {
            Some(PlanetToExplorer::GenerateResourceResponse {
                resource: generate_resource(state, generator, resource),
            })
        }

        ExplorerToPlanet::CombineResourceRequest { msg, .. } => {
            Some(PlanetToExplorer::CombineResourceResponse {
                complex_response: combine_resource(ai, state, combinator, msg),
            })
        }

        ExplorerToPlanet::AvailableEnergyCellRequest { .. } => {
            Some(PlanetToExplorer::AvailableEnergyCellResponse {
                available_cells: if state.cell(0).is_charged() { 1 } else { 0 },
            })
        }
    }
}

/// Returns the available Basic Resources set of the planet
fn supported_resources(generator: &Generator) -> HashSet<BasicResourceType> {
    generator.all_available_recipes()
}

/// Returns the available Complex Resources set of the planet
fn supported_combinations(combinator: &Combinator) -> HashSet<ComplexResourceType> {
    combinator.all_available_recipes()
}

/// Return the optional Basic resource generated
fn generate_resource(
    state: &mut PlanetState,
    generator: &Generator,
    to_generate: BasicResourceType,
) -> Option<BasicResource> {
    match to_generate {
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
    }
}

/// Returns the optional complex resource created
fn combine_resource(
    ai: &Ai,
    state: &mut PlanetState,
    combinator: &Combinator,
    msg: ComplexResourceRequest,
) -> Result<ComplexResource, (String, GenericResource, GenericResource)> {
    if ai.random_mode() {
        todo!()
    }

    match msg {
        ComplexResourceRequest::Water(r1, r2) => combinator
            .make_water(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Water)
            .map_err(|(s, r1, r2)| {
                (
                    s,
                    GenericResource::BasicResources(BasicResource::Hydrogen(r1)),
                    GenericResource::BasicResources(BasicResource::Oxygen(r2)),
                )
            }),

        ComplexResourceRequest::Diamond(r1, r2) => combinator
            .make_diamond(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Diamond)
            .map_err(|(s, r1, r2)| {
                (
                    s,
                    GenericResource::BasicResources(BasicResource::Carbon(r1)),
                    GenericResource::BasicResources(BasicResource::Carbon(r2)),
                )
            }),

        ComplexResourceRequest::Life(r1, r2) => combinator
            .make_life(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Life)
            .map_err(|(s, r1, r2)| {
                (
                    s,
                    GenericResource::ComplexResources(ComplexResource::Water(r1)),
                    GenericResource::BasicResources(BasicResource::Carbon(r2)),
                )
            }),

        ComplexResourceRequest::Robot(r1, r2) => combinator
            .make_robot(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Robot)
            .map_err(|(s, r1, r2)| {
                (
                    s,
                    GenericResource::BasicResources(BasicResource::Silicon(r1)),
                    GenericResource::ComplexResources(ComplexResource::Life(r2)),
                )
            }),

        ComplexResourceRequest::Dolphin(r1, r2) => combinator
            .make_dolphin(r1, r2, state.cell_mut(0))
            .map(ComplexResource::Dolphin)
            .map_err(|(s, r1, r2)| {
                (
                    s,
                    GenericResource::ComplexResources(ComplexResource::Water(r1)),
                    GenericResource::ComplexResources(ComplexResource::Life(r2)),
                )
            }),

        ComplexResourceRequest::AIPartner(r1, r2) => combinator
            .make_aipartner(r1, r2, state.cell_mut(0))
            .map(ComplexResource::AIPartner)
            .map_err(|(s, r1, r2)| {
                (
                    s,
                    GenericResource::ComplexResources(ComplexResource::Robot(r1)),
                    GenericResource::ComplexResources(ComplexResource::Diamond(r2)),
                )
            }),
    }
}
