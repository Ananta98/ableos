use crate::kernel_state::cpuid::{cpuid, EDXFeatureBitMasks};

/// Returns whether or not apic is available on this processor.
/// Sets register eax to 1, then queries cpuid
pub fn has_apic() -> bool {
	// Note: this is not where it sets eax to 1.
	// This is.
	let (_, _, _, edx) = cpuid(1, 0, 0, 0);
	(edx & EDXFeatureBitMasks::APIC as u32) > 0
}
