pub mod rd300nx;
pub mod live_set;
pub mod layers;
pub mod tones;
pub mod system;

//TODO (TYPES) handle types better than all u8 (across entire project)
//TODO (TYPES) factor out common sets of settings between layers and live set common/etc, or system and live set
//TODO (TYPES) make microtune a map of non-default values (are there any other arrays which are really maps?)
//TODO (TYPES) look for other sections of json which are overly verbose and contain basically default data and figure out what to do with them
