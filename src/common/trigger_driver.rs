#[derive(PartialEq, Clone)]
pub enum TriggerMode {
	RisingEdge,
	FallingEdge,
	BothEdges,
}

pub fn should_trigger(old_clock_state: bool, new_clock_state: bool, trigger_mode: TriggerMode) -> bool {
	if new_clock_state != old_clock_state && (trigger_mode == TriggerMode::BothEdges
		|| trigger_mode == TriggerMode::RisingEdge && new_clock_state 
		|| trigger_mode == TriggerMode::FallingEdge && !new_clock_state) {
		return true;
	}

	false
}