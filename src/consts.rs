//! Useful constants and static values.
#![allow(unused_mut)]

use crate::{
	game_data::{Attribute, TargetType, Weapon},
	ids::*,
	player::Race,
};
use std::collections::HashMap;

/// Default in-game speed modifier (on **Faster** game speed).
/// See [page on liquipedia](https://liquipedia.net/starcraft2/Game_Speed) for more info.
pub const GAME_SPEED: f32 = 1.4;
/// Frames per second, calculated by `16 (default frames per second) * 1.4 (game speed)`.
pub const FRAMES_PER_SECOND: f32 = 22.4;

/// Units under effect of raven's anit-armor missile have this buff.
/// It reduces armor and shield armor by 3 (armor can be negative at this point).
#[cfg(windows)]
pub const ANTI_ARMOR_BUFF: BuffId = BuffId::RavenShredderMissileArmorReductionUISubtruct;
#[cfg(unix)]
pub const ANTI_ARMOR_BUFF: BuffId = BuffId::RavenShredderMissileArmorReduction;
/// Unit targeted by raven's anit-armor missile have this buff.
pub const ANTI_ARMOR_TARGET: BuffId = BuffId::RavenShredderMissileTint;
/// Units disabled by raven's interference matrix have this buff.
pub const INTERFERENCE_MATRIX_BUFF: BuffId = BuffId::RavenScramblerMissile;

#[cfg(windows)]
pub(crate) const INHIBITOR_IDS: [UnitTypeId; 6] = [
	UnitTypeId::InhibitorZoneSmall,
	UnitTypeId::InhibitorZoneMedium,
	UnitTypeId::InhibitorZoneLarge,
	UnitTypeId::InhibitorZoneFlyingSmall,
	UnitTypeId::InhibitorZoneFlyingMedium,
	UnitTypeId::InhibitorZoneFlyingLarge,
];
#[cfg(unix)]
pub(crate) const INHIBITOR_IDS: [UnitTypeId; 3] = [
	UnitTypeId::InhibitorZoneSmall,
	UnitTypeId::InhibitorZoneMedium,
	UnitTypeId::InhibitorZoneLarge,
];

/// Structured values, specific for each race.
#[derive(Clone)]
pub struct RaceValues {
	/// Default townhall without any upgrades, which can be built by a worker.
	pub start_townhall: UnitTypeId,
	/// All possible forms of townhall.
	pub townhalls: Vec<UnitTypeId>,
	/// Building used to extract gas from vespene geysers.
	pub gas: UnitTypeId,
	/// Building used to extract gas from rich vespene geysers.
	pub rich_gas: UnitTypeId,
	/// Supply provider for this race.
	pub supply: UnitTypeId,
	/// Worker of this race.
	pub worker: UnitTypeId,
}
impl Default for RaceValues {
	fn default() -> Self {
		Self {
			start_townhall: UnitTypeId::NotAUnit,
			townhalls: Vec::new(),
			gas: UnitTypeId::NotAUnit,
			rich_gas: UnitTypeId::NotAUnit,
			supply: UnitTypeId::NotAUnit,
			worker: UnitTypeId::NotAUnit,
		}
	}
}

type BonusesForTarget = HashMap<TargetType, BonusesByAttribute>;
type BonusesByAttribute = (Option<u32>, HashMap<Attribute, u32>);

lazy_static! {
	/// [`RaceValues`] mapped to each race.
	pub static ref RACE_VALUES: HashMap<Race, RaceValues> = hashmap![
		Race::Terran => RaceValues {
			start_townhall: UnitTypeId::CommandCenter,
			townhalls: vec![
				UnitTypeId::CommandCenter,
				UnitTypeId::OrbitalCommand,
				UnitTypeId::PlanetaryFortress,
				UnitTypeId::CommandCenterFlying,
				UnitTypeId::OrbitalCommandFlying,
			],
			gas: UnitTypeId::Refinery,
			rich_gas: UnitTypeId::RefineryRich,
			supply: UnitTypeId::SupplyDepot,
			worker: UnitTypeId::SCV,
		},
		Race::Zerg => RaceValues {
			start_townhall: UnitTypeId::Hatchery,
			townhalls: vec![UnitTypeId::Hatchery, UnitTypeId::Lair, UnitTypeId::Hive],
			gas: UnitTypeId::Extractor,
			rich_gas: UnitTypeId::ExtractorRich,
			supply: UnitTypeId::Overlord,
			worker: UnitTypeId::Drone,
		},
		Race::Protoss => RaceValues {
			start_townhall: UnitTypeId::Nexus,
			townhalls: vec![UnitTypeId::Nexus],
			gas: UnitTypeId::Assimilator,
			rich_gas: UnitTypeId::AssimilatorRich,
			supply: UnitTypeId::Pylon,
			worker: UnitTypeId::Probe,
		},
	];
	pub(crate) static ref TECH_ALIAS: HashMap<UnitTypeId, Vec<UnitTypeId>> = hashmap![
		UnitTypeId::Assimilator => vec![UnitTypeId::AssimilatorRich],
		UnitTypeId::AssimilatorRich => vec![UnitTypeId::Assimilator],
		UnitTypeId::Barracks => vec![UnitTypeId::BarracksFlying],
		UnitTypeId::BarracksFlying => vec![UnitTypeId::Barracks],
		UnitTypeId::CommandCenter => vec![
			UnitTypeId::CommandCenterFlying,
			UnitTypeId::OrbitalCommand,
			UnitTypeId::OrbitalCommandFlying,
			UnitTypeId::PlanetaryFortress,
		],
		UnitTypeId::CommandCenterFlying => vec![
			UnitTypeId::CommandCenter,
			UnitTypeId::OrbitalCommand,
			UnitTypeId::OrbitalCommandFlying,
			UnitTypeId::PlanetaryFortress,
		],
		UnitTypeId::OrbitalCommand => vec![
			UnitTypeId::CommandCenter,
			UnitTypeId::CommandCenterFlying,
			UnitTypeId::OrbitalCommandFlying,
			UnitTypeId::PlanetaryFortress,
		],
		UnitTypeId::OrbitalCommandFlying => vec![
			UnitTypeId::CommandCenter,
			UnitTypeId::CommandCenterFlying,
			UnitTypeId::OrbitalCommand,
			UnitTypeId::PlanetaryFortress,
		],
		UnitTypeId::PlanetaryFortress => vec![
			UnitTypeId::CommandCenter,
			UnitTypeId::CommandCenterFlying,
			UnitTypeId::OrbitalCommand,
			UnitTypeId::OrbitalCommandFlying,
		],
		UnitTypeId::CreepTumorBurrowed => vec![UnitTypeId::CreepTumor, UnitTypeId::CreepTumorQueen],
		UnitTypeId::CreepTumorQueen => vec![UnitTypeId::CreepTumor, UnitTypeId::CreepTumorBurrowed],
		UnitTypeId::Hatchery => vec![UnitTypeId::Lair, UnitTypeId::Hive],
		UnitTypeId::Lair => vec![UnitTypeId::Hatchery, UnitTypeId::Hive],
		UnitTypeId::Hive => vec![UnitTypeId::Hatchery, UnitTypeId::Lair],
		UnitTypeId::LiberatorAG => vec![UnitTypeId::Liberator],
		UnitTypeId::Liberator => vec![UnitTypeId::LiberatorAG],
		UnitTypeId::Extractor => vec![UnitTypeId::ExtractorRich],
		UnitTypeId::ExtractorRich => vec![UnitTypeId::Extractor],
		UnitTypeId::Spire => vec![UnitTypeId::GreaterSpire],
		UnitTypeId::GreaterSpire => vec![UnitTypeId::Spire],
		UnitTypeId::OverlordTransport => vec![UnitTypeId::Overlord],
		UnitTypeId::Overlord => vec![UnitTypeId::OverlordTransport],
		UnitTypeId::Overseer => vec![UnitTypeId::OverseerSiegeMode],
		UnitTypeId::OverseerSiegeMode => vec![UnitTypeId::Overseer],
		UnitTypeId::FactoryFlying => vec![UnitTypeId::Factory],
		UnitTypeId::StarportFlying => vec![UnitTypeId::Starport],
		UnitTypeId::Reactor => vec![
			UnitTypeId::BarracksReactor,
			UnitTypeId::FactoryReactor,
			UnitTypeId::StarportReactor,
		],
		UnitTypeId::TechLab => vec![
			UnitTypeId::BarracksTechLab,
			UnitTypeId::FactoryTechLab,
			UnitTypeId::StarportTechLab,
		],
		UnitTypeId::BarracksReactor => vec![UnitTypeId::Reactor],
		UnitTypeId::BarracksTechLab => vec![UnitTypeId::TechLab],
		UnitTypeId::FactoryReactor => vec![UnitTypeId::Reactor],
		UnitTypeId::FactoryTechLab => vec![UnitTypeId::TechLab],
		UnitTypeId::StarportReactor => vec![UnitTypeId::Reactor],
		UnitTypeId::StarportTechLab => vec![UnitTypeId::TechLab],
		UnitTypeId::PylonOvercharged => vec![UnitTypeId::Pylon],
		UnitTypeId::Pylon => vec![UnitTypeId::PylonOvercharged],
		UnitTypeId::QueenBurrowed => vec![UnitTypeId::Queen],
		UnitTypeId::Queen => vec![UnitTypeId::QueenBurrowed],
		UnitTypeId::Refinery => vec![UnitTypeId::RefineryRich],
		UnitTypeId::RefineryRich => vec![UnitTypeId::Refinery],
		UnitTypeId::SiegeTank => vec![UnitTypeId::SiegeTankSieged],
		UnitTypeId::SiegeTankSieged => vec![UnitTypeId::SiegeTank],
		UnitTypeId::SupplyDepot => vec![UnitTypeId::SupplyDepotLowered],
		UnitTypeId::SupplyDepotLowered => vec![UnitTypeId::SupplyDepot],
		UnitTypeId::Thor => vec![UnitTypeId::ThorAP],
		UnitTypeId::ThorAP => vec![UnitTypeId::Thor],
		UnitTypeId::Viking => vec![UnitTypeId::VikingFighter, UnitTypeId::VikingAssault],
		UnitTypeId::VikingFighter => vec![UnitTypeId::Viking, UnitTypeId::VikingAssault],
		UnitTypeId::VikingAssault => vec![UnitTypeId::Viking, UnitTypeId::VikingFighter],
		UnitTypeId::Gateway => vec![UnitTypeId::WarpGate],
		UnitTypeId::WarpGate => vec![UnitTypeId::Gateway],
		UnitTypeId::WarpPrism => vec![UnitTypeId::WarpPrismPhasing],
		UnitTypeId::WarpPrismPhasing => vec![UnitTypeId::WarpPrism],
		UnitTypeId::WidowMine => vec![UnitTypeId::WidowMineBurrowed],
		UnitTypeId::WidowMineBurrowed => vec![UnitTypeId::WidowMine],
	];
	pub(crate) static ref UNIT_ALIAS: HashMap<UnitTypeId, UnitTypeId> = hashmap![
		UnitTypeId::Adept => UnitTypeId::AdeptPhaseShift,
		UnitTypeId::AdeptPhaseShift => UnitTypeId::Adept,
		UnitTypeId::Assimilator => UnitTypeId::AssimilatorRich,
		UnitTypeId::AssimilatorRich => UnitTypeId::Assimilator,
		UnitTypeId::Baneling => UnitTypeId::BanelingBurrowed,
		UnitTypeId::BanelingBurrowed => UnitTypeId::Baneling,
		UnitTypeId::Barracks => UnitTypeId::BarracksFlying,
		UnitTypeId::BarracksFlying => UnitTypeId::Barracks,
		UnitTypeId::Changeling => UnitTypeId::ChangelingMarine,
		UnitTypeId::ChangelingMarine => UnitTypeId::Changeling,
		UnitTypeId::Changeling => UnitTypeId::ChangelingMarineShield,
		UnitTypeId::ChangelingMarineShield => UnitTypeId::Changeling,
		UnitTypeId::Changeling => UnitTypeId::ChangelingZealot,
		UnitTypeId::ChangelingZealot => UnitTypeId::Changeling,
		UnitTypeId::Changeling => UnitTypeId::ChangelingZergling,
		UnitTypeId::ChangelingZergling => UnitTypeId::Changeling,
		UnitTypeId::Changeling => UnitTypeId::ChangelingZerglingWings,
		UnitTypeId::ChangelingZerglingWings => UnitTypeId::Changeling,
		UnitTypeId::CommandCenter => UnitTypeId::CommandCenterFlying,
		UnitTypeId::CommandCenterFlying => UnitTypeId::CommandCenter,
		UnitTypeId::CreepTumor => UnitTypeId::CreepTumorBurrowed,
		UnitTypeId::CreepTumorBurrowed => UnitTypeId::CreepTumor,
		UnitTypeId::CreepTumor => UnitTypeId::CreepTumorQueen,
		UnitTypeId::CreepTumorQueen => UnitTypeId::CreepTumor,
		UnitTypeId::Drone => UnitTypeId::DroneBurrowed,
		UnitTypeId::DroneBurrowed => UnitTypeId::Drone,
		UnitTypeId::Extractor => UnitTypeId::ExtractorRich,
		UnitTypeId::ExtractorRich => UnitTypeId::Extractor,
		UnitTypeId::Factory => UnitTypeId::FactoryFlying,
		UnitTypeId::FactoryFlying => UnitTypeId::Factory,
		UnitTypeId::Hydralisk => UnitTypeId::HydraliskBurrowed,
		UnitTypeId::HydraliskBurrowed => UnitTypeId::Hydralisk,
		UnitTypeId::Infestor => UnitTypeId::InfestorBurrowed,
		UnitTypeId::InfestorBurrowed => UnitTypeId::Infestor,
		UnitTypeId::InfestorTerran => UnitTypeId::InfestorTerranBurrowed,
		UnitTypeId::InfestorTerranBurrowed => UnitTypeId::InfestorTerran,
		UnitTypeId::Liberator => UnitTypeId::LiberatorAG,
		UnitTypeId::LiberatorAG => UnitTypeId::Liberator,
		UnitTypeId::LocustMP => UnitTypeId::LocustMPFlying,
		UnitTypeId::LocustMPFlying => UnitTypeId::LocustMP,
		UnitTypeId::LurkerMP => UnitTypeId::LurkerMPBurrowed,
		UnitTypeId::LurkerMPBurrowed => UnitTypeId::LurkerMP,
		UnitTypeId::Observer => UnitTypeId::ObserverSiegeMode,
		UnitTypeId::ObserverSiegeMode => UnitTypeId::Observer,
		UnitTypeId::OrbitalCommand => UnitTypeId::OrbitalCommandFlying,
		UnitTypeId::OrbitalCommandFlying => UnitTypeId::OrbitalCommand,
		UnitTypeId::Overseer => UnitTypeId::OverseerSiegeMode,
		UnitTypeId::OverseerSiegeMode => UnitTypeId::Overseer,
		UnitTypeId::Pylon => UnitTypeId::PylonOvercharged,
		UnitTypeId::PylonOvercharged => UnitTypeId::Pylon,
		UnitTypeId::Queen => UnitTypeId::QueenBurrowed,
		UnitTypeId::QueenBurrowed => UnitTypeId::Queen,
		UnitTypeId::Ravager => UnitTypeId::RavagerBurrowed,
		UnitTypeId::RavagerBurrowed => UnitTypeId::Ravager,
		UnitTypeId::Refinery => UnitTypeId::RefineryRich,
		UnitTypeId::RefineryRich => UnitTypeId::Refinery,
		UnitTypeId::Roach => UnitTypeId::RoachBurrowed,
		UnitTypeId::RoachBurrowed => UnitTypeId::Roach,
		UnitTypeId::SiegeTank => UnitTypeId::SiegeTankSieged,
		UnitTypeId::SiegeTankSieged => UnitTypeId::SiegeTank,
		UnitTypeId::SpineCrawler => UnitTypeId::SpineCrawlerUprooted,
		UnitTypeId::SpineCrawlerUprooted => UnitTypeId::SpineCrawler,
		UnitTypeId::SporeCrawler => UnitTypeId::SporeCrawlerUprooted,
		UnitTypeId::SporeCrawlerUprooted => UnitTypeId::SporeCrawler,
		UnitTypeId::Starport => UnitTypeId::StarportFlying,
		UnitTypeId::StarportFlying => UnitTypeId::Starport,
		UnitTypeId::SupplyDepot => UnitTypeId::SupplyDepotLowered,
		UnitTypeId::SupplyDepotLowered => UnitTypeId::SupplyDepot,
		UnitTypeId::SwarmHostMP => UnitTypeId::SwarmHostBurrowedMP,
		UnitTypeId::SwarmHostBurrowedMP => UnitTypeId::SwarmHostMP,
		UnitTypeId::Thor => UnitTypeId::ThorAP,
		UnitTypeId::ThorAP => UnitTypeId::Thor,
		UnitTypeId::Ultralisk => UnitTypeId::UltraliskBurrowed,
		UnitTypeId::UltraliskBurrowed => UnitTypeId::Ultralisk,
		UnitTypeId::VikingFighter => UnitTypeId::VikingAssault,
		UnitTypeId::VikingAssault => UnitTypeId::VikingFighter,
		UnitTypeId::WarpPrismPhasing => UnitTypeId::WarpPrism,
		UnitTypeId::WarpPrism => UnitTypeId::WarpPrismPhasing,
		UnitTypeId::WidowMine => UnitTypeId::WidowMineBurrowed,
		UnitTypeId::WidowMineBurrowed => UnitTypeId::WidowMine,
		UnitTypeId::Zergling => UnitTypeId::ZerglingBurrowed,
		UnitTypeId::ZerglingBurrowed => UnitTypeId::Zergling,
	];
	/// Tech requirements mapped to different units.
	///
	/// Basic usage:
	/// ```
	/// if let Some(requirment) = TECH_REQUIREMENTS.get(unit_type) {
	///     /* do what you like */
	/// }
	/// ```
	pub static ref TECH_REQUIREMENTS: HashMap<UnitTypeId, UnitTypeId> = hashmap![
		// Terran
		UnitTypeId::MissileTurret => UnitTypeId::EngineeringBay,
		UnitTypeId::SensorTower => UnitTypeId::EngineeringBay,
		UnitTypeId::PlanetaryFortress => UnitTypeId::EngineeringBay,
		UnitTypeId::Barracks => UnitTypeId::SupplyDepot,
		UnitTypeId::OrbitalCommand => UnitTypeId::Barracks,
		UnitTypeId::Bunker => UnitTypeId::Barracks,
		UnitTypeId::Ghost => UnitTypeId::GhostAcademy,
		UnitTypeId::GhostAcademy => UnitTypeId::Barracks,
		UnitTypeId::Factory => UnitTypeId::Barracks,
		UnitTypeId::Armory => UnitTypeId::Factory,
		UnitTypeId::HellionTank => UnitTypeId::Armory,
		UnitTypeId::Thor => UnitTypeId::Armory,
		UnitTypeId::Starport => UnitTypeId::Factory,
		UnitTypeId::FusionCore => UnitTypeId::Starport,
		UnitTypeId::Battlecruiser => UnitTypeId::FusionCore,
		// Protoss
		UnitTypeId::PhotonCannon => UnitTypeId::Forge,
		UnitTypeId::CyberneticsCore => UnitTypeId::Gateway,
		UnitTypeId::Sentry => UnitTypeId::CyberneticsCore,
		UnitTypeId::Stalker => UnitTypeId::CyberneticsCore,
		UnitTypeId::Adept => UnitTypeId::CyberneticsCore,
		UnitTypeId::TwilightCouncil => UnitTypeId::CyberneticsCore,
		UnitTypeId::ShieldBattery => UnitTypeId::CyberneticsCore,
		UnitTypeId::TemplarArchive => UnitTypeId::TwilightCouncil,
		UnitTypeId::DarkShrine => UnitTypeId::TwilightCouncil,
		UnitTypeId::HighTemplar => UnitTypeId::TemplarArchive,
		UnitTypeId::DarkTemplar => UnitTypeId::DarkShrine,
		UnitTypeId::Stargate => UnitTypeId::CyberneticsCore,
		UnitTypeId::Tempest => UnitTypeId::FleetBeacon,
		UnitTypeId::Carrier => UnitTypeId::FleetBeacon,
		UnitTypeId::Mothership => UnitTypeId::FleetBeacon,
		UnitTypeId::RoboticsFacility => UnitTypeId::CyberneticsCore,
		UnitTypeId::RoboticsBay => UnitTypeId::RoboticsFacility,
		UnitTypeId::Colossus => UnitTypeId::RoboticsBay,
		UnitTypeId::Disruptor => UnitTypeId::RoboticsBay,
		// Zerg
		UnitTypeId::Zergling => UnitTypeId::SpawningPool,
		UnitTypeId::Queen => UnitTypeId::SpawningPool,
		UnitTypeId::RoachWarren => UnitTypeId::SpawningPool,
		UnitTypeId::BanelingNest => UnitTypeId::SpawningPool,
		UnitTypeId::SpineCrawler => UnitTypeId::SpawningPool,
		UnitTypeId::SporeCrawler => UnitTypeId::SpawningPool,
		UnitTypeId::Roach => UnitTypeId::RoachWarren,
		UnitTypeId::Baneling => UnitTypeId::BanelingNest,
		UnitTypeId::Lair => UnitTypeId::SpawningPool,
		UnitTypeId::Overseer => UnitTypeId::Lair,
		UnitTypeId::OverlordTransport => UnitTypeId::Lair,
		UnitTypeId::InfestationPit => UnitTypeId::Lair,
		UnitTypeId::Infestor => UnitTypeId::InfestationPit,
		UnitTypeId::SwarmHostMP => UnitTypeId::InfestationPit,
		UnitTypeId::HydraliskDen => UnitTypeId::Lair,
		UnitTypeId::Hydralisk => UnitTypeId::HydraliskDen,
		UnitTypeId::LurkerDenMP => UnitTypeId::HydraliskDen,
		UnitTypeId::LurkerMP => UnitTypeId::LurkerDenMP,
		UnitTypeId::Spire => UnitTypeId::Lair,
		UnitTypeId::Mutalisk => UnitTypeId::Spire,
		UnitTypeId::Corruptor => UnitTypeId::Spire,
		UnitTypeId::NydusNetwork => UnitTypeId::Lair,
		UnitTypeId::Hive => UnitTypeId::InfestationPit,
		UnitTypeId::Viper => UnitTypeId::Hive,
		UnitTypeId::UltraliskCavern => UnitTypeId::Hive,
		UnitTypeId::GreaterSpire => UnitTypeId::Hive,
		UnitTypeId::BroodLord => UnitTypeId::GreaterSpire,
	];
	/// Producers mapped to different units.
	pub static ref PRODUCERS: HashMap<UnitTypeId, UnitTypeId> = hashmap![
		UnitTypeId::Adept => UnitTypeId::Gateway,
		UnitTypeId::Armory => UnitTypeId::SCV,
		UnitTypeId::Assimilator => UnitTypeId::Probe,
		UnitTypeId::AutoTurret => UnitTypeId::Raven,
		UnitTypeId::Baneling => UnitTypeId::Zergling,
		UnitTypeId::BanelingNest => UnitTypeId::Drone,
		UnitTypeId::Banshee => UnitTypeId::Starport,
		UnitTypeId::Barracks => UnitTypeId::SCV,
		UnitTypeId::Battlecruiser => UnitTypeId::Starport,
		UnitTypeId::BroodLord => UnitTypeId::Corruptor,
		UnitTypeId::Bunker => UnitTypeId::SCV,
		UnitTypeId::Carrier => UnitTypeId::Stargate,
		UnitTypeId::Changeling => UnitTypeId::Overseer,
		UnitTypeId::Colossus => UnitTypeId::RoboticsFacility,
		UnitTypeId::CommandCenter => UnitTypeId::SCV,
		UnitTypeId::Corruptor => UnitTypeId::Larva,
		UnitTypeId::CreepTumor => UnitTypeId::Queen,
		UnitTypeId::CreepTumorQueen => UnitTypeId::Queen,
		UnitTypeId::CyberneticsCore => UnitTypeId::Probe,
		UnitTypeId::Cyclone => UnitTypeId::Factory,
		UnitTypeId::DarkShrine => UnitTypeId::Probe,
		UnitTypeId::DarkTemplar => UnitTypeId::Gateway,
		UnitTypeId::Disruptor => UnitTypeId::RoboticsFacility,
		UnitTypeId::Drone => UnitTypeId::Larva,
		UnitTypeId::EngineeringBay => UnitTypeId::SCV,
		UnitTypeId::EvolutionChamber => UnitTypeId::Drone,
		UnitTypeId::Extractor => UnitTypeId::Drone,
		UnitTypeId::Factory => UnitTypeId::SCV,
		UnitTypeId::FleetBeacon => UnitTypeId::Probe,
		UnitTypeId::Forge => UnitTypeId::Probe,
		UnitTypeId::FusionCore => UnitTypeId::SCV,
		UnitTypeId::Gateway => UnitTypeId::Probe,
		UnitTypeId::Ghost => UnitTypeId::Barracks,
		UnitTypeId::GhostAcademy => UnitTypeId::SCV,
		UnitTypeId::GreaterSpire => UnitTypeId::Spire,
		UnitTypeId::Hatchery => UnitTypeId::Drone,
		UnitTypeId::Hellion => UnitTypeId::Factory,
		UnitTypeId::HellionTank => UnitTypeId::Factory,
		UnitTypeId::HighTemplar => UnitTypeId::Gateway,
		UnitTypeId::Hive => UnitTypeId::Lair,
		UnitTypeId::Hydralisk => UnitTypeId::Larva,
		UnitTypeId::HydraliskDen => UnitTypeId::Drone,
		UnitTypeId::Immortal => UnitTypeId::RoboticsFacility,
		UnitTypeId::InfestationPit => UnitTypeId::Drone,
		UnitTypeId::Infestor => UnitTypeId::Larva,
		UnitTypeId::Lair => UnitTypeId::Hatchery,
		UnitTypeId::Liberator => UnitTypeId::Starport,
		UnitTypeId::LocustMPFlying => UnitTypeId::SwarmHostMP,
		UnitTypeId::LurkerDenMP => UnitTypeId::Drone,
		UnitTypeId::LurkerMP => UnitTypeId::Hydralisk,
		UnitTypeId::Marauder => UnitTypeId::Barracks,
		UnitTypeId::Marine => UnitTypeId::Barracks,
		UnitTypeId::Medivac => UnitTypeId::Starport,
		UnitTypeId::MissileTurret => UnitTypeId::SCV,
		UnitTypeId::Mothership => UnitTypeId::Nexus,
		UnitTypeId::Mutalisk => UnitTypeId::Larva,
		UnitTypeId::Nexus => UnitTypeId::Probe,
		UnitTypeId::NydusCanal => UnitTypeId::NydusNetwork,
		UnitTypeId::NydusNetwork => UnitTypeId::Drone,
		UnitTypeId::Observer => UnitTypeId::RoboticsFacility,
		UnitTypeId::Oracle => UnitTypeId::Stargate,
		UnitTypeId::OracleStasisTrap => UnitTypeId::Oracle,
		UnitTypeId::OrbitalCommand => UnitTypeId::CommandCenter,
		UnitTypeId::Overlord => UnitTypeId::Larva,
		UnitTypeId::OverlordTransport => UnitTypeId::Overlord,
		UnitTypeId::Overseer => UnitTypeId::Overlord,
		UnitTypeId::Phoenix => UnitTypeId::Stargate,
		UnitTypeId::PhotonCannon => UnitTypeId::Probe,
		UnitTypeId::PlanetaryFortress => UnitTypeId::CommandCenter,
		UnitTypeId::Probe => UnitTypeId::Nexus,
		UnitTypeId::Pylon => UnitTypeId::Probe,
		UnitTypeId::Queen => UnitTypeId::Hatchery,
		UnitTypeId::Ravager => UnitTypeId::Roach,
		UnitTypeId::Raven => UnitTypeId::Starport,
		UnitTypeId::Reaper => UnitTypeId::Barracks,
		UnitTypeId::Refinery => UnitTypeId::SCV,
		UnitTypeId::Roach => UnitTypeId::Larva,
		UnitTypeId::RoachWarren => UnitTypeId::Drone,
		UnitTypeId::RoboticsBay => UnitTypeId::Probe,
		UnitTypeId::RoboticsFacility => UnitTypeId::Probe,
		UnitTypeId::SCV => UnitTypeId::CommandCenter,
		UnitTypeId::SensorTower => UnitTypeId::SCV,
		UnitTypeId::Sentry => UnitTypeId::Gateway,
		UnitTypeId::ShieldBattery => UnitTypeId::Probe,
		UnitTypeId::SiegeTank => UnitTypeId::Factory,
		UnitTypeId::SpawningPool => UnitTypeId::Drone,
		UnitTypeId::SpineCrawler => UnitTypeId::Drone,
		UnitTypeId::Spire => UnitTypeId::Drone,
		UnitTypeId::SporeCrawler => UnitTypeId::Drone,
		UnitTypeId::Stalker => UnitTypeId::Gateway,
		UnitTypeId::Stargate => UnitTypeId::Probe,
		UnitTypeId::Starport => UnitTypeId::SCV,
		UnitTypeId::SupplyDepot => UnitTypeId::SCV,
		UnitTypeId::SwarmHostMP => UnitTypeId::Larva,
		UnitTypeId::Tempest => UnitTypeId::Stargate,
		UnitTypeId::TemplarArchive => UnitTypeId::Probe,
		UnitTypeId::Thor => UnitTypeId::Factory,
		UnitTypeId::TwilightCouncil => UnitTypeId::Probe,
		UnitTypeId::Ultralisk => UnitTypeId::Larva,
		UnitTypeId::UltraliskCavern => UnitTypeId::Drone,
		UnitTypeId::VikingFighter => UnitTypeId::Starport,
		UnitTypeId::Viper => UnitTypeId::Larva,
		UnitTypeId::VoidRay => UnitTypeId::Stargate,
		UnitTypeId::WarpPrism => UnitTypeId::RoboticsFacility,
		UnitTypeId::WidowMine => UnitTypeId::Factory,
		UnitTypeId::Zealot => UnitTypeId::Gateway,
		UnitTypeId::Zergling => UnitTypeId::Larva,
	];
	/// Producers and their alias mapped to different units.
	pub static ref ALL_PRODUCERS: HashMap<UnitTypeId, Vec<UnitTypeId>> = hashmap![
		UnitTypeId::Adept => vec![UnitTypeId::Gateway, UnitTypeId::WarpGate],
		UnitTypeId::Armory => vec![UnitTypeId::SCV],
		UnitTypeId::Assimilator => vec![UnitTypeId::Probe],
		UnitTypeId::AutoTurret => vec![UnitTypeId::Raven],
		UnitTypeId::Baneling => vec![UnitTypeId::Zergling],
		UnitTypeId::BanelingNest => vec![UnitTypeId::Drone],
		UnitTypeId::Banshee => vec![UnitTypeId::Starport],
		UnitTypeId::Barracks => vec![UnitTypeId::SCV],
		UnitTypeId::Battlecruiser => vec![UnitTypeId::Starport],
		UnitTypeId::BroodLord => vec![UnitTypeId::Corruptor],
		UnitTypeId::Bunker => vec![UnitTypeId::SCV],
		UnitTypeId::Carrier => vec![UnitTypeId::Stargate],
		UnitTypeId::Changeling => vec![UnitTypeId::Overseer, UnitTypeId::OverseerSiegeMode],
		UnitTypeId::Colossus => vec![UnitTypeId::RoboticsFacility],
		UnitTypeId::CommandCenter => vec![UnitTypeId::SCV],
		UnitTypeId::Corruptor => vec![UnitTypeId::Larva],
		UnitTypeId::CreepTumor => vec![
			UnitTypeId::CreepTumor,
			UnitTypeId::CreepTumorBurrowed,
			UnitTypeId::CreepTumorQueen,
			UnitTypeId::Queen,
		],
		UnitTypeId::CreepTumorQueen => vec![UnitTypeId::Queen],
		UnitTypeId::CyberneticsCore => vec![UnitTypeId::Probe],
		UnitTypeId::Cyclone => vec![UnitTypeId::Factory],
		UnitTypeId::DarkShrine => vec![UnitTypeId::Probe],
		UnitTypeId::DarkTemplar => vec![UnitTypeId::Gateway, UnitTypeId::WarpGate],
		UnitTypeId::Disruptor => vec![UnitTypeId::RoboticsFacility],
		UnitTypeId::Drone => vec![UnitTypeId::Larva],
		UnitTypeId::EngineeringBay => vec![UnitTypeId::SCV],
		UnitTypeId::EvolutionChamber => vec![UnitTypeId::Drone],
		UnitTypeId::Extractor => vec![UnitTypeId::Drone],
		UnitTypeId::Factory => vec![UnitTypeId::SCV],
		UnitTypeId::FleetBeacon => vec![UnitTypeId::Probe],
		UnitTypeId::Forge => vec![UnitTypeId::Probe],
		UnitTypeId::FusionCore => vec![UnitTypeId::SCV],
		UnitTypeId::Gateway => vec![UnitTypeId::Probe],
		UnitTypeId::Ghost => vec![UnitTypeId::Barracks],
		UnitTypeId::GhostAcademy => vec![UnitTypeId::SCV],
		UnitTypeId::GreaterSpire => vec![UnitTypeId::Spire],
		UnitTypeId::Hatchery => vec![UnitTypeId::Drone],
		UnitTypeId::Hellion => vec![UnitTypeId::Factory],
		UnitTypeId::HellionTank => vec![UnitTypeId::Factory],
		UnitTypeId::HighTemplar => vec![UnitTypeId::Gateway, UnitTypeId::WarpGate],
		UnitTypeId::Hive => vec![UnitTypeId::Lair],
		UnitTypeId::Hydralisk => vec![UnitTypeId::Larva],
		UnitTypeId::HydraliskDen => vec![UnitTypeId::Drone],
		UnitTypeId::Immortal => vec![UnitTypeId::RoboticsFacility],
		UnitTypeId::InfestationPit => vec![UnitTypeId::Drone],
		UnitTypeId::Infestor => vec![UnitTypeId::Larva],
		UnitTypeId::Lair => vec![UnitTypeId::Hatchery],
		UnitTypeId::Liberator => vec![UnitTypeId::Starport],
		UnitTypeId::LocustMPFlying => vec![UnitTypeId::SwarmHostBurrowedMP, UnitTypeId::SwarmHostMP],
		UnitTypeId::LurkerDenMP => vec![UnitTypeId::Drone],
		UnitTypeId::LurkerMP => vec![UnitTypeId::Hydralisk],
		UnitTypeId::Marauder => vec![UnitTypeId::Barracks],
		UnitTypeId::Marine => vec![UnitTypeId::Barracks],
		UnitTypeId::Medivac => vec![UnitTypeId::Starport],
		UnitTypeId::MissileTurret => vec![UnitTypeId::SCV],
		UnitTypeId::Mothership => vec![UnitTypeId::Nexus],
		UnitTypeId::Mutalisk => vec![UnitTypeId::Larva],
		UnitTypeId::Nexus => vec![UnitTypeId::Probe],
		UnitTypeId::NydusCanal => vec![UnitTypeId::NydusNetwork],
		UnitTypeId::NydusNetwork => vec![UnitTypeId::Drone],
		UnitTypeId::Observer => vec![UnitTypeId::RoboticsFacility],
		UnitTypeId::Oracle => vec![UnitTypeId::Stargate],
		UnitTypeId::OracleStasisTrap => vec![UnitTypeId::Oracle],
		UnitTypeId::OrbitalCommand => vec![UnitTypeId::CommandCenter],
		UnitTypeId::Overlord => vec![UnitTypeId::Larva],
		UnitTypeId::OverlordTransport => vec![UnitTypeId::Overlord],
		UnitTypeId::Overseer => vec![UnitTypeId::Overlord, UnitTypeId::OverlordTransport],
		UnitTypeId::Phoenix => vec![UnitTypeId::Stargate],
		UnitTypeId::PhotonCannon => vec![UnitTypeId::Probe],
		UnitTypeId::PlanetaryFortress => vec![UnitTypeId::CommandCenter],
		UnitTypeId::Probe => vec![UnitTypeId::Nexus],
		UnitTypeId::Pylon => vec![UnitTypeId::Probe],
		UnitTypeId::Queen => vec![UnitTypeId::Hatchery, UnitTypeId::Hive, UnitTypeId::Lair],
		UnitTypeId::Ravager => vec![UnitTypeId::Roach],
		UnitTypeId::Raven => vec![UnitTypeId::Starport],
		UnitTypeId::Reaper => vec![UnitTypeId::Barracks],
		UnitTypeId::Refinery => vec![UnitTypeId::SCV],
		UnitTypeId::Roach => vec![UnitTypeId::Larva],
		UnitTypeId::RoachWarren => vec![UnitTypeId::Drone],
		UnitTypeId::RoboticsBay => vec![UnitTypeId::Probe],
		UnitTypeId::RoboticsFacility => vec![UnitTypeId::Probe],
		UnitTypeId::SCV => vec![
			UnitTypeId::CommandCenter,
			UnitTypeId::OrbitalCommand,
			UnitTypeId::PlanetaryFortress,
		],
		UnitTypeId::SensorTower => vec![UnitTypeId::SCV],
		UnitTypeId::Sentry => vec![UnitTypeId::Gateway, UnitTypeId::WarpGate],
		UnitTypeId::ShieldBattery => vec![UnitTypeId::Probe],
		UnitTypeId::SiegeTank => vec![UnitTypeId::Factory],
		UnitTypeId::SpawningPool => vec![UnitTypeId::Drone],
		UnitTypeId::SpineCrawler => vec![UnitTypeId::Drone],
		UnitTypeId::Spire => vec![UnitTypeId::Drone],
		UnitTypeId::SporeCrawler => vec![UnitTypeId::Drone],
		UnitTypeId::Stalker => vec![UnitTypeId::Gateway, UnitTypeId::WarpGate],
		UnitTypeId::Stargate => vec![UnitTypeId::Probe],
		UnitTypeId::Starport => vec![UnitTypeId::SCV],
		UnitTypeId::SupplyDepot => vec![UnitTypeId::SCV],
		UnitTypeId::SwarmHostMP => vec![UnitTypeId::Larva],
		UnitTypeId::Tempest => vec![UnitTypeId::Stargate],
		UnitTypeId::TemplarArchive => vec![UnitTypeId::Probe],
		UnitTypeId::Thor => vec![UnitTypeId::Factory],
		UnitTypeId::TwilightCouncil => vec![UnitTypeId::Probe],
		UnitTypeId::Ultralisk => vec![UnitTypeId::Larva],
		UnitTypeId::UltraliskCavern => vec![UnitTypeId::Drone],
		UnitTypeId::VikingFighter => vec![UnitTypeId::Starport],
		UnitTypeId::Viper => vec![UnitTypeId::Larva],
		UnitTypeId::VoidRay => vec![UnitTypeId::Stargate],
		UnitTypeId::WarpPrism => vec![UnitTypeId::RoboticsFacility],
		UnitTypeId::WidowMine => vec![UnitTypeId::Factory],
		UnitTypeId::Zealot => vec![UnitTypeId::Gateway, UnitTypeId::WarpGate],
		UnitTypeId::Zergling => vec![UnitTypeId::Larva],
	];
	/// Researchers mapped to upgrades.
	pub static ref RESEARCHERS: HashMap<UpgradeId, UnitTypeId> = {
		let mut map = hashmap![
			UpgradeId::AdeptPiercingAttack => UnitTypeId::TwilightCouncil,
			UpgradeId::AnabolicSynthesis => UnitTypeId::UltraliskCavern,
			UpgradeId::BansheeCloak => UnitTypeId::StarportTechLab,
			UpgradeId::BansheeSpeed => UnitTypeId::StarportTechLab,
			UpgradeId::BattlecruiserEnableSpecializations => UnitTypeId::FusionCore,
			UpgradeId::BlinkTech => UnitTypeId::TwilightCouncil,
			UpgradeId::Burrow => UnitTypeId::Hive,
			UpgradeId::Charge => UnitTypeId::TwilightCouncil,
			UpgradeId::ChitinousPlating => UnitTypeId::UltraliskCavern,
			UpgradeId::CycloneLockOnDamageUpgrade => UnitTypeId::FactoryTechLab,
			UpgradeId::DarkTemplarBlinkUpgrade => UnitTypeId::DarkShrine,
			UpgradeId::DiggingClaws => UnitTypeId::LurkerDenMP,
			UpgradeId::DrillClaws => UnitTypeId::FactoryTechLab,
			UpgradeId::EvolveGroovedSpines => UnitTypeId::HydraliskDen,
			UpgradeId::EvolveMuscularAugments => UnitTypeId::HydraliskDen,
			UpgradeId::ExtendedThermalLance => UnitTypeId::RoboticsBay,
			UpgradeId::GraviticDrive => UnitTypeId::RoboticsBay,
			UpgradeId::HighCapacityBarrels => UnitTypeId::FactoryTechLab,
			UpgradeId::HiSecAutoTracking => UnitTypeId::EngineeringBay,
			UpgradeId::InfestorEnergyUpgrade => UnitTypeId::InfestationPit,
			UpgradeId::LiberatorMorph => UnitTypeId::StarportTechLab,
			UpgradeId::MedivacIncreaseSpeedBoost => UnitTypeId::StarportTechLab,
			UpgradeId::NeuralParasite => UnitTypeId::InfestationPit,
			UpgradeId::ObserverGraviticBooster => UnitTypeId::RoboticsBay,
			UpgradeId::Overlordspeed => UnitTypeId::Hive,
			UpgradeId::PersonalCloaking => UnitTypeId::GhostAcademy,
			UpgradeId::PhoenixRangeUpgrade => UnitTypeId::FleetBeacon,
			UpgradeId::ProtossAirArmorsLevel1 => UnitTypeId::CyberneticsCore,
			UpgradeId::ProtossAirArmorsLevel2 => UnitTypeId::CyberneticsCore,
			UpgradeId::ProtossAirArmorsLevel3 => UnitTypeId::CyberneticsCore,
			UpgradeId::ProtossAirWeaponsLevel1 => UnitTypeId::CyberneticsCore,
			UpgradeId::ProtossAirWeaponsLevel2 => UnitTypeId::CyberneticsCore,
			UpgradeId::ProtossAirWeaponsLevel3 => UnitTypeId::CyberneticsCore,
			UpgradeId::ProtossGroundArmorsLevel1 => UnitTypeId::Forge,
			UpgradeId::ProtossGroundArmorsLevel2 => UnitTypeId::Forge,
			UpgradeId::ProtossGroundArmorsLevel3 => UnitTypeId::Forge,
			UpgradeId::ProtossGroundWeaponsLevel1 => UnitTypeId::Forge,
			UpgradeId::ProtossGroundWeaponsLevel2 => UnitTypeId::Forge,
			UpgradeId::ProtossGroundWeaponsLevel3 => UnitTypeId::Forge,
			UpgradeId::ProtossShieldsLevel1 => UnitTypeId::Forge,
			UpgradeId::ProtossShieldsLevel2 => UnitTypeId::Forge,
			UpgradeId::ProtossShieldsLevel3 => UnitTypeId::Forge,
			UpgradeId::PsiStormTech => UnitTypeId::TemplarArchive,
			UpgradeId::PunisherGrenades => UnitTypeId::BarracksTechLab,
			UpgradeId::RavenCorvidReactor => UnitTypeId::StarportTechLab,
			UpgradeId::ShieldWall => UnitTypeId::BarracksTechLab,
			UpgradeId::SmartServos => UnitTypeId::FactoryTechLab,
			UpgradeId::Stimpack => UnitTypeId::BarracksTechLab,
			UpgradeId::TerranBuildingArmor => UnitTypeId::EngineeringBay,
			UpgradeId::TerranInfantryArmorsLevel1 => UnitTypeId::EngineeringBay,
			UpgradeId::TerranInfantryArmorsLevel2 => UnitTypeId::EngineeringBay,
			UpgradeId::TerranInfantryArmorsLevel3 => UnitTypeId::EngineeringBay,
			UpgradeId::TerranInfantryWeaponsLevel1 => UnitTypeId::EngineeringBay,
			UpgradeId::TerranInfantryWeaponsLevel2 => UnitTypeId::EngineeringBay,
			UpgradeId::TerranInfantryWeaponsLevel3 => UnitTypeId::EngineeringBay,
			UpgradeId::TerranShipWeaponsLevel1 => UnitTypeId::Armory,
			UpgradeId::TerranShipWeaponsLevel2 => UnitTypeId::Armory,
			UpgradeId::TerranShipWeaponsLevel3 => UnitTypeId::Armory,
			UpgradeId::TerranVehicleWeaponsLevel1 => UnitTypeId::Armory,
			UpgradeId::TerranVehicleWeaponsLevel2 => UnitTypeId::Armory,
			UpgradeId::TerranVehicleWeaponsLevel3 => UnitTypeId::Armory,
			UpgradeId::TerranVehicleAndShipArmorsLevel1 => UnitTypeId::Armory,
			UpgradeId::TerranVehicleAndShipArmorsLevel2 => UnitTypeId::Armory,
			UpgradeId::TerranVehicleAndShipArmorsLevel3 => UnitTypeId::Armory,
			UpgradeId::WarpGateResearch => UnitTypeId::CyberneticsCore,
			UpgradeId::ZergFlyerArmorsLevel1 => UnitTypeId::GreaterSpire,
			UpgradeId::ZergFlyerArmorsLevel2 => UnitTypeId::GreaterSpire,
			UpgradeId::ZergFlyerArmorsLevel3 => UnitTypeId::GreaterSpire,
			UpgradeId::ZergFlyerWeaponsLevel1 => UnitTypeId::GreaterSpire,
			UpgradeId::ZergFlyerWeaponsLevel2 => UnitTypeId::GreaterSpire,
			UpgradeId::ZergFlyerWeaponsLevel3 => UnitTypeId::GreaterSpire,
			UpgradeId::ZergGroundArmorsLevel1 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergGroundArmorsLevel2 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergGroundArmorsLevel3 => UnitTypeId::EvolutionChamber,
			UpgradeId::Zerglingattackspeed => UnitTypeId::SpawningPool,
			UpgradeId::Zerglingmovementspeed => UnitTypeId::SpawningPool,
			UpgradeId::ZergMeleeWeaponsLevel1 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergMeleeWeaponsLevel2 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergMeleeWeaponsLevel3 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergMissileWeaponsLevel1 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergMissileWeaponsLevel2 => UnitTypeId::EvolutionChamber,
			UpgradeId::ZergMissileWeaponsLevel3 => UnitTypeId::EvolutionChamber,
		];
		#[cfg(windows)]
		{
			map.reserve(1);
			map.insert(UpgradeId::EnhancedShockwaves, UnitTypeId::GhostAcademy);
		}
		map
	};

	pub(crate) static ref DAMAGE_BONUS_PER_UPGRADE: HashMap<UnitTypeId, BonusesForTarget> = hashmap![
		// Protoss
		UnitTypeId::Probe => hashmap![TargetType::Ground => (Some(0), hashmap![])],
		UnitTypeId::Adept => hashmap![TargetType::Ground => (None, hashmap![Attribute::Light => 1])],
		UnitTypeId::Stalker => hashmap![TargetType::Any => (None, hashmap![Attribute::Armored => 1])],
		UnitTypeId::DarkTemplar => hashmap![TargetType::Ground => (Some(5), hashmap![])],
		UnitTypeId::Archon => hashmap![TargetType::Any => (Some(3), hashmap![Attribute::Biological => 1])],
		UnitTypeId::Immortal => hashmap![TargetType::Ground => (Some(2), hashmap![Attribute::Armored => 3])],
		UnitTypeId::Colossus => hashmap![TargetType::Ground => (None, hashmap![Attribute::Light => 1])],
		UnitTypeId::Oracle => hashmap![TargetType::Ground => (Some(0), hashmap![])],
		UnitTypeId::Tempest => hashmap![
			TargetType::Ground => (Some(4), hashmap![]),
			TargetType::Air => (Some(3), hashmap![Attribute::Massive => 2]),
		],
		// Terran
		UnitTypeId::SCV => hashmap![TargetType::Ground => (Some(0), hashmap![])],
		UnitTypeId::Marauder => hashmap![TargetType::Ground => (None, hashmap![Attribute::Armored => 1])],
		UnitTypeId::Ghost => hashmap![TargetType::Any => (None, hashmap![Attribute::Light => 1])],
		UnitTypeId::Hellion => hashmap![TargetType::Ground => (None, hashmap![Attribute::Light => 1])],
		UnitTypeId::HellionTank => hashmap![TargetType::Ground => (Some(2), hashmap![Attribute::Light => 1])],
		UnitTypeId::Cyclone => hashmap![TargetType::Any => (Some(2), hashmap![])],
		UnitTypeId::SiegeTank => hashmap![TargetType::Ground => (Some(2), hashmap![Attribute::Armored => 1])],
		UnitTypeId::SiegeTankSieged => hashmap![TargetType::Ground => (Some(4), hashmap![Attribute::Armored => 1])],
		UnitTypeId::Thor => hashmap![
			TargetType::Ground => (Some(3), hashmap![]),
			TargetType::Air => (None, hashmap![Attribute::Light => 1]),
		],
		UnitTypeId::ThorAP => hashmap![
			TargetType::Ground => (Some(3), hashmap![]),
			TargetType::Air => (Some(3), hashmap![Attribute::Massive => 1]),
		],
		UnitTypeId::VikingAssault => hashmap![TargetType::Ground => (None, hashmap![Attribute::Mechanical => 1])],
		UnitTypeId::LiberatorAG => hashmap![TargetType::Ground => (Some(5), hashmap![])],
		// Zerg
		UnitTypeId::Drone => hashmap![TargetType::Ground => (Some(0), hashmap![])],
		UnitTypeId::Baneling => hashmap![
			TargetType::Ground => (Some(2), hashmap![Attribute::Light => 2, Attribute::Structure => 3])
		],
		UnitTypeId::BanelingBurrowed => hashmap![
			TargetType::Ground => (Some(2), hashmap![Attribute::Light => 2, Attribute::Structure => 3])
		],
		UnitTypeId::BanelingCocoon => hashmap![
			TargetType::Ground => (Some(2), hashmap![Attribute::Light => 2, Attribute::Structure => 3])
		],
		UnitTypeId::Roach => hashmap![TargetType::Ground => (Some(2), hashmap![])],
		UnitTypeId::Ravager => hashmap![TargetType::Ground => (Some(2), hashmap![])],
		UnitTypeId::RavagerCocoon => hashmap![TargetType::Ground => (Some(2), hashmap![])],
		UnitTypeId::LurkerMPBurrowed => hashmap![TargetType::Ground => (Some(2), hashmap![Attribute::Armored => 1])],
		UnitTypeId::Ultralisk => hashmap![TargetType::Ground => (Some(3), hashmap![])],
		UnitTypeId::Corruptor => hashmap![TargetType::Air => (None, hashmap![Attribute::Massive => 1])],
		UnitTypeId::BroodLord => hashmap![TargetType::Ground => (Some(2), hashmap![])],
	];

	pub(crate) static ref SPEED_UPGRADES: HashMap<UnitTypeId, (UpgradeId, f32)> = {
		let mut map = hashmap![
			// Terran
			UnitTypeId::Banshee => (UpgradeId::BansheeSpeed, 1.3636),
			// Protoss
			UnitTypeId::Zealot => (UpgradeId::Charge, 1.5),
			UnitTypeId::Observer => (UpgradeId::ObserverGraviticBooster, 2.0),
			UnitTypeId::WarpPrism => (UpgradeId::GraviticDrive, 1.3),
			// Zerg
			UnitTypeId::Overlord => (UpgradeId::Overlordspeed, 2.915),
			UnitTypeId::Overseer => (UpgradeId::Overlordspeed, 1.8015),
			UnitTypeId::Zergling => (UpgradeId::Zerglingmovementspeed, 1.6),
			UnitTypeId::Baneling => (UpgradeId::CentrificalHooks, 1.18),
			UnitTypeId::Roach => (UpgradeId::GlialReconstitution, 1.333_333_4),
			UnitTypeId::LurkerMP => (UpgradeId::DiggingClaws, 1.1),
		];
		if cfg!(windows) {
			map.reserve(2);
			map.insert(UnitTypeId::Medivac, (UpgradeId::MedivacRapidDeployment, 1.18));
			map.insert(UnitTypeId::VoidRay, (UpgradeId::VoidRaySpeedUpgrade, 1.328));
		}
		map
	};
	pub(crate) static ref OFF_CREEP_SPEED_UPGRADES: HashMap<UnitTypeId, (UpgradeId, f32)> = hashmap![
		UnitTypeId::Hydralisk => (UpgradeId::EvolveMuscularAugments, 1.25),
		UnitTypeId::Ultralisk => (UpgradeId::AnabolicSynthesis, 1.2),
	];
	pub(crate) static ref SPEED_ON_CREEP: HashMap<UnitTypeId, f32> = hashmap![
		UnitTypeId::Queen => 2.67,
		UnitTypeId::Zergling => 1.3,
		UnitTypeId::Baneling => 1.3,
		UnitTypeId::Roach => 1.3,
		UnitTypeId::Ravager => 1.3,
		UnitTypeId::Hydralisk => 1.3,
		UnitTypeId::LurkerMP => 1.3,
		UnitTypeId::Ultralisk => 1.3,
		UnitTypeId::Infestor => 1.3,
		UnitTypeId::InfestorTerran => 1.3,
		UnitTypeId::SwarmHostMP => 1.3,
		UnitTypeId::LocustMP => 1.4,
		UnitTypeId::SpineCrawler => 2.5,
		UnitTypeId::SporeCrawler => 2.5,
	];

	pub(crate) static ref WARPGATE_ABILITIES: HashMap<UnitTypeId, AbilityId> = hashmap![
		UnitTypeId::Zealot => AbilityId::WarpGateTrainZealot,
		UnitTypeId::Stalker => AbilityId::WarpGateTrainStalker,
		UnitTypeId::HighTemplar => AbilityId::WarpGateTrainHighTemplar,
		UnitTypeId::DarkTemplar => AbilityId::WarpGateTrainDarkTemplar,
		UnitTypeId::Sentry => AbilityId::WarpGateTrainSentry,
		UnitTypeId::Adept => AbilityId::TrainWarpAdept,
	];

	pub(crate) static ref MISSED_WEAPONS: HashMap<UnitTypeId, Vec<Weapon>> = hashmap![
		UnitTypeId::Baneling => vec![Weapon {
			target: TargetType::Ground,
			damage: 20,
			damage_bonus: vec![(Attribute::Light, 15), (Attribute::Structure, 60)],
			attacks: 1,
			range: 2.2,
			speed: 1.0,
		}],
		UnitTypeId::Battlecruiser => vec![
			Weapon {
				target: TargetType::Ground,
				damage: 8,
				damage_bonus: vec![],
				attacks: 1,
				range: 6.0,
				speed: 0.224,
			},
			Weapon {
				target: TargetType::Air,
				damage: 5,
				damage_bonus: vec![],
				attacks: 1,
				range: 6.0,
				speed: 0.224,
			},
		],
		UnitTypeId::Sentry => vec![Weapon {
			target: TargetType::Any,
			damage: 6,
			damage_bonus: vec![],
			attacks: 1,
			range: 5.0,
			speed: 0.994,
		}],
		UnitTypeId::VoidRay => vec![Weapon {
			target: TargetType::Any,
			damage: 6,
			damage_bonus: vec![],
			attacks: 1,
			range: 6.0,
			speed: 0.504,
		}],
		UnitTypeId::Bunker => vec![Weapon {
			target: TargetType::Any,
			damage: 6, // Marine damage
			damage_bonus: vec![],
			attacks: 4,   // 4 Marines inside
			range: 6.0,   // Marine range + 1
			speed: 0.854, // Marine cooldown
		}],
		UnitTypeId::Carrier => vec![Weapon {
			target: TargetType::Any,
			damage: 5,
			damage_bonus: vec![],
			attacks: 16,
			range: 8.0, // Interceptors launch range
			speed: 2.996,
		}],
		UnitTypeId::Oracle => vec![Weapon {
			target: TargetType::Ground,
			damage: 15,
			damage_bonus: vec![(Attribute::Light, 7)],
			attacks: 1,
			range: 4.0,
			speed: 0.854,
		}],
		UnitTypeId::WidowMineBurrowed => vec![Weapon {
			target: TargetType::Any,
			damage: 125,
			damage_bonus: vec![],
			attacks: 1,
			range: 5.0,
			speed: 1.0,
		}],
	];
	/// Radiuses of Inhibitor Zones mapped to their ids.
	pub static ref INHIBITOR_ZONE_RADIUS: HashMap<UnitTypeId, f32> = {
		let mut map = hashmap![
			UnitTypeId::InhibitorZoneSmall => 4.0,
			UnitTypeId::InhibitorZoneMedium => 5.0,
			UnitTypeId::InhibitorZoneLarge => 6.0,
		];
		#[cfg(windows)]
		{
			map.reserve(3);
			map.insert(UnitTypeId::InhibitorZoneFlyingSmall, 4.0);
			map.insert(UnitTypeId::InhibitorZoneFlyingMedium, 5.0);
			map.insert(UnitTypeId::InhibitorZoneFlyingLarge, 6.0);
		}
		map
	};
	pub(crate) static ref SPEED_BUFFS: HashMap<BuffId, f32> = {
		let mut map = hashmap![
			BuffId::Stimpack => 1.5,
			BuffId::StimpackMarauder => 1.5,
			BuffId::ChargeUp => if cfg!(windows) { 2.8 } else { 2.2 },
			BuffId::DutchMarauderSlow => 0.5,
			BuffId::TimeWarpProduction => 0.5,
			BuffId::FungalGrowth => 0.25,
			BuffId::InhibitorZoneTemporalField => 0.65,
		];
		#[cfg(windows)]
		{
			map.reserve(3);
			map.insert(BuffId::InhibitorZoneFlyingTemporalField, 0.65);
			map.insert(BuffId::AccelerationZoneTemporalField, 1.35);
			map.insert(BuffId::AccelerationZoneFlyingTemporalField, 1.35);
		}
		map
	};
}