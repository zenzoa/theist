const setupAddTagDialog = () => {
	tauri_listen('open_add_tag_dialog', openAddTagDialog)

	document.getElementById('add-tag-button').addEventListener('click', openAddTagDialog)

	document.getElementById('add-tag-close-button').addEventListener('click', closeAddTagDialog)
	document.getElementById('add-tag-cancel-button').addEventListener('click', closeAddTagDialog)

	document.getElementById('add-agent-tag-button').addEventListener('click', () => {
		tauri_invoke('add_agent_tag', {})
		closeAddTagDialog()
	})
	document.getElementById('add-egg-tag-button').addEventListener('click', () => {
		tauri_invoke('add_egg_tag', {})
		closeAddTagDialog()
	})
	document.getElementById('add-gb-tag-button').addEventListener('click', () => {
		tauri_invoke('add_gb_tag', {})
		closeAddTagDialog()
	})
}

const openAddTagDialog = () => {
	document.getElementById('add-tag-dialog').classList.add('open')
	document.getElementById('add-agent-tag-button').focus()
}

const closeAddTagDialog = () => {
	document.getElementById('add-tag-dialog').classList.remove('open')
}
