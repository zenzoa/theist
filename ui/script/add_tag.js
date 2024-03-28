class AddTagDialog {
	static isOpen() {
		return document.getElementById('add-tag-dialog').classList.contains('open')
	}

	static open() {
		document.getElementById('add-tag-dialog').classList.add('open')
		document.getElementById('add-agent-tag-button').focus()
	}

	static close() {
		document.getElementById('add-tag-dialog').classList.remove('open')
	}

	static setup() {
		document.getElementById('add-tag-button')
			.addEventListener('click', AddTagDialog.open)

		document.getElementById('add-tag-close-button')
			.addEventListener('click', AddTagDialog.close)

		document.getElementById('add-tag-cancel-button')
			.addEventListener('click', AddTagDialog.close)

		document.getElementById('add-agent-tag-button').addEventListener('click', () => {
			tauri_invoke('add_agent_tag', {})
			AddTagDialog.close()
		})

		document.getElementById('add-egg-tag-button').addEventListener('click', () => {
			tauri_invoke('add_egg_tag', {})
			AddTagDialog.close()
		})

		document.getElementById('add-gb-tag-button').addEventListener('click', () => {
			tauri_invoke('add_gb_tag', {})
			AddTagDialog.close()
		})

		tauri_listen('show_add_tag_dialog', AddTagDialog.open)
	}
}
