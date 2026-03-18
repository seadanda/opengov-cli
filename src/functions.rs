use anyhow::{Context, Result};
use std::fs;

// Check what the user entered for the proposal. If it is just call data, return it back. Otherwise,
// we expect a path to a file that contains the call data. Read that in and return it.
pub(crate) fn get_proposal_bytes(proposal: String) -> Result<Vec<u8>> {
	let proposal = proposal.as_str();
	if proposal.starts_with("0x") {
		// This is just call data
		hex::decode(proposal.trim_start_matches("0x")).context("Invalid hex in proposal call data")
	} else {
		// This is a file path
		let contents = fs::read_to_string(proposal)
			.with_context(|| format!("Failed to read proposal file: {}", proposal))?;
		hex::decode(contents.as_str().trim_start_matches("0x"))
			.context("Invalid hex in proposal file contents")
	}
}
