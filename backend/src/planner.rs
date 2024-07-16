// use std::{
//     cmp::Ordering,
//     fmt::{self, Display, Formatter},
//     fs::{read_to_string, write},
//     sync::{Arc, RwLock},
// };

// use crate::{
//     astek::Astek,
//     helpers::{InternalError, IntrastekError},
// };
// use serde::{Deserialize, Serialize};
// use serde_json::{from_str, to_string};

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Planner {
//     pub activities: Vec<Activity>,
// }

// fn sort_asteks_by_time_on_activities(
//     asteks: &[Arc<RwLock<Astek>>],
//     activity: Activities,
// ) -> Vec<Arc<RwLock<Astek>>> {
//     let mut asteks = asteks.to_owned();
//     asteks.sort_by(|a, b| {
//         let a = a.read().unwrap().get_time_spent_for_activity(activity);
//         let b = b.read().unwrap().get_time_spent_for_activity(activity);
//         match a.partial_cmp(&b) {
//             Some(order) => order,
//             None => Ordering::Equal,
//         }
//     });
//     asteks
// }

// #[derive(Debug, Clone)]
// struct MissingAsteksError {
//     activity: Activities,
//     module: Option<Module>,
// }

// impl IntrastekError for MissingAsteksError {
//     fn get_code(&self) -> u16 {
//         480
//     }

//     fn get_message(&self) -> String {
//         match self.module {
//             Some(module) => format!(
//                 "Not enough asteks for activity {} on module {}",
//                 self.activity, module
//             ),
//             None => format!("Not enough asteks for activity {}", self.activity),
//         }
//     }
// }
// impl Planner {
//     pub fn new() -> Self {
//         Planner {
//             activities: Vec::new(),
//         }
//     }

//     pub fn from_file(file: &str) -> Result<Self, String> {
//         let file = read_to_string(file).map_err(|e| e.to_string())?;
//         let planner: Planner = from_str(&file).map_err(|e| e.to_string())?;
//         Ok(planner)
//     }

//     pub fn save_to_file(self, path: &str) -> Result<(), String> {
//         let file = to_string(&self).map_err(|e| e.to_string())?;
//         write(path, file).map_err(|e| e.to_string())?;
//         Ok(())
//     }

//     pub fn add_activity(&mut self, activity: Activity) {
//         self.activities.push(activity);
//     }

//     fn get_available_asteks(
//         asteks: &[Arc<RwLock<Astek>>],
//         activity: &Activity,
//     ) -> Result<Vec<Arc<RwLock<Astek>>>, Box<dyn IntrastekError>> {
//         let mut res = asteks.to_owned();

//         res.retain(|astek| match astek.as_ref().read() {
//             Ok(a) => a.is_available(&activity.interval),
//             Err(_) => false,
//         });

//         Ok(res)
//     }

//     fn pick_asteks(
//         activity: &mut Activity,
//         available_asteks: Vec<Arc<RwLock<Astek>>>,
//     ) -> Result<(), Box<dyn IntrastekError>> {
//         let sorted = sort_asteks_by_time_on_activities(&available_asteks, activity.activity);
//         (0..activity.needed_asteks).try_for_each(|i| match sorted.get(i as usize) {
//             Some(astek) => match astek.write() {
//                 Ok(mut atk) => {
//                     atk.assign(activity.clone());
//                     activity.add_astek(atk.id);
//                     Ok(())
//                 }
//                 Err(_) => Err::<(), Box<dyn IntrastekError>>(Box::new(InternalError)),
//             },
//             None => Err::<(), Box<dyn IntrastekError>>(Box::new(MissingAsteksError {
//                 activity: activity.activity,
//                 module: activity.module,
//             })),
//         })
//     }

//     pub fn compute(
//         &mut self,
//         asteks: &[Arc<RwLock<Astek>>],
//     ) -> Result<(), Box<dyn IntrastekError>> {
//         self.activities.iter_mut().try_for_each(|activity| {
//             let available_asteks = Planner::get_available_asteks(asteks, activity)?;
//             Planner::pick_asteks(activity, available_asteks)?;
//             Ok(())
//         })
//     }
// }

// impl Display for Planner {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         self.activities
//             .iter()
//             .try_for_each(|activity| write!(f, "{}", activity))
//     }
// }
