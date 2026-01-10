#![allow(clippy::pedantic)]

use common_game::components::planet::Planet;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use immutable_cosmic_borrow::create_planet;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// Helper functions to test the planet AI behaviour

#[allow(dead_code)]
pub fn create_test_planet() -> (
    Planet,
    (
        crossbeam_channel::Sender<OrchestratorToPlanet>,
        crossbeam_channel::Receiver<PlanetToOrchestrator>,
    ),
    crossbeam_channel::Sender<ExplorerToPlanet>,
) {
    // Channel 1: Orchestrator -> Planet
    let (tx_orchestrator_to_planet, rx_orchestrator_to_planet) =
        crossbeam_channel::bounded::<OrchestratorToPlanet>(1);
    // Channel 2: Planet -> Orchestrator
    let (tx_planet_to_orchestrator, rx_planet_to_orchestrator) =
        crossbeam_channel::bounded::<PlanetToOrchestrator>(1);

    // Channel 3: Explorer -> Planet
    let (tx_explorer_to_planet, rx_explorer_to_planet) =
        crossbeam_channel::bounded::<ExplorerToPlanet>(1);
    // Channel 4: Planet -> Explorer
    let (_tx_planet_to_explorer, _rx_planet_to_explorer) =
        crossbeam_channel::bounded::<PlanetToExplorer>(1);

    let planet = create_planet(
        true,
        0.0,
        0.0,
        Duration::from_millis(100),
        Duration::from_secs(1),
        0,
        (rx_orchestrator_to_planet, tx_planet_to_orchestrator),
        rx_explorer_to_planet,
    );

    assert!(planet.is_ok(), "Planet creation failed");
    (
        planet.unwrap(),
        (tx_orchestrator_to_planet, rx_planet_to_orchestrator),
        tx_explorer_to_planet,
    )
}
pub fn orchestrator_start_planet(
    tx_orchestrator: &crossbeam_channel::Sender<OrchestratorToPlanet>,
    rx_orchestrator: &crossbeam_channel::Receiver<PlanetToOrchestrator>,
) {
    tx_orchestrator
        .send(OrchestratorToPlanet::StartPlanetAI)
        .expect("Orchestrator failed to send");
    thread::sleep(Duration::from_millis(50));

    rx_orchestrator
        .recv_timeout(Duration::from_millis(200))
        .expect("Orchestrator failed to receive");
}

#[allow(dead_code)]
pub fn orchestrator_kill_planet(
    tx_orchestrator: &crossbeam_channel::Sender<OrchestratorToPlanet>,
    rx_orchestrator: &crossbeam_channel::Receiver<PlanetToOrchestrator>,
) {
    let _ = orchestrator_send(
        tx_orchestrator,
        rx_orchestrator,
        OrchestratorToPlanet::KillPlanet,
    );
    thread::sleep(Duration::from_millis(200));
}

#[allow(dead_code)]
pub fn orchestrator_stop_planet(
    tx_orchestrator: &crossbeam_channel::Sender<OrchestratorToPlanet>,
    rx_orchestrator: &crossbeam_channel::Receiver<PlanetToOrchestrator>,
) {
    tx_orchestrator
        .send(OrchestratorToPlanet::StopPlanetAI)
        .expect("Orchestrator failed to send");
    thread::sleep(Duration::from_millis(50));
    rx_orchestrator
        .recv_timeout(Duration::from_millis(200))
        .expect("Orchestrator failed to receive");

    orchestrator_send(
        tx_orchestrator,
        rx_orchestrator,
        OrchestratorToPlanet::KillPlanet,
    );
}
pub fn orchestrator_send(
    tx: &crossbeam_channel::Sender<OrchestratorToPlanet>,
    rx: &crossbeam_channel::Receiver<PlanetToOrchestrator>,
    msg: OrchestratorToPlanet,
) -> PlanetToOrchestrator {
    tx.send(msg).expect("Orchestrator failed to send");
    thread::sleep(Duration::from_millis(50));

    rx.recv_timeout(Duration::from_millis(200))
        .expect("Orchestrator failed to receive")
}

#[allow(dead_code)]
pub fn explorer_send(
    tx: &crossbeam_channel::Sender<ExplorerToPlanet>,
    rx: &crossbeam_channel::Receiver<PlanetToExplorer>,
    msg: ExplorerToPlanet,
) -> PlanetToExplorer {
    tx.send(msg).expect("Explorer failed to send");
    thread::sleep(Duration::from_millis(50));

    rx.recv_timeout(Duration::from_millis(2000))
        .expect("Explorer failed to receive")
}
pub fn start_thread(mut planet: Planet) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || planet.run())
}
