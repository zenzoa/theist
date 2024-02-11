const Tauri = window.__TAURI__
const tauri_listen = Tauri.event.listen
const tauri_invoke = Tauri.core.invoke

let tags = []
let selectedTag = 0

let dependencies = []
let lastSelectedDependency = 0
let selectedDependencies = []
let checkedDependencies = []

window.addEventListener('load', () => {

	// disable context menu
	document.body.addEventListener('contextmenu', event => {
		event.preventDefault()
		return false
	}, false)

	document.getElementById('new-file-button').addEventListener('click', () => {
		tauri_invoke('new_file')
	})

	document.getElementById('open-file-button').addEventListener('click', () => {
		tauri_invoke('open_file')
	})

	document.getElementById('save-file-button').addEventListener('click', () => {
		tauri_invoke('save_file')
	})

	document.getElementById('save-as-file-button').addEventListener('click', () => {
		tauri_invoke('save_file_as')
	})

	document.getElementById('undo-button').addEventListener('click', () => {
		tauri_invoke('undo')
	})

	document.getElementById('redo-button').addEventListener('click', () => {
		tauri_invoke('redo')
	})

	document.getElementById('add-dependency-button').addEventListener('click', () => {
		tauri_invoke('add_dependency')
	})

	document.getElementById('extract-dependency-button').addEventListener('click', () => {
		tauri_invoke('extract_dependency', { selectedDependencies })
	})

	document.getElementById('reload-dependency-button').addEventListener('click', () => {
		tauri_invoke('reload_dependency', { selectedDependencies })
	})

	document.getElementById('remove-dependency-button').addEventListener('click', () => {
		tauri_invoke('remove_dependency', { selectedDependencies })
	})

	tauri_listen('update_tag_list', updateTagList)
	tauri_listen('update_tag_info', updateTagInfo)

	tauri_listen('update_dependency_list', updateDependencyList)
	tauri_listen('update_checked_dependencies', updateCheckedDependencies)

	tauri_listen('show_notification', showNotification)

	tauri_listen('show_spinner', showSpinner)
	tauri_listen('hide_spinner', hideSpinner)

	tauri_listen('enable_save_button', enableButton.bind(this, 'save-file-button'))
	tauri_listen('enable_undo_button', enableButton.bind(this, 'undo-button'))
	tauri_listen('enable_redo_button', enableButton.bind(this, 'redo-button'))

	tauri_listen('set_theme', setTheme)

	setupAddTagDialog()

	document.body.addEventListener('keydown', (event) => {
		const KEY = event.key.toUpperCase()
		const ONLY = !event.ctrlKey && !event.shiftKey && !event.altKey
		const CTRL = event.ctrlKey && !event.shiftKey && !event.altKey
		const CTRL_SHIFT = event.ctrlKey && event.shiftKey && !event.altKey

		if (CTRL && KEY === 'Q') {
			event.preventDefault()
			tauri_invoke('try_quit')

		} else if (CTRL && KEY === 'N') {
			event.preventDefault()
			tauri_invoke('new_file')

		} else if (CTRL && KEY === 'O') {
			event.preventDefault()
			tauri_invoke('open_file')

		} else if (CTRL && KEY === 'S') {
			event.preventDefault()
			tauri_invoke('save_file')

		} else if (CTRL_SHIFT && KEY === 'S') {
			event.preventDefault()
			tauri_invoke('save_as_file')

		} else if (CTRL && KEY === 'Z') {
			event.preventDefault()
			tauri_invoke('undo')

		} else if (CTRL_SHIFT && KEY === 'Z') {
			event.preventDefault()
			tauri_invoke('redo')

		} else if (CTRL_SHIFT && KEY === 'N') {
			event.preventDefault()
			openAddTagDialog()

		} else if (CTRL && KEY === 'A') {
			event.preventDefault()
			selectAllDependencies()

		} else if (CTRL && KEY === 'D') {
			event.preventDefault()
			deselectAllDependencies()

		} else if (KEY === 'ESCAPE' && document.getElementById('add-tag-dialog').classList.contains('open')){
			closeAddTagDialog()
		}
	})
})

const showNotification = (event) => {
	const notificationEl = document.getElementById('notification')
	notificationEl.innerText = event.payload
	notificationEl.classList.add('on')
	setTimeout(() => notificationEl.classList.remove('on'), 2000)
}

const showSpinner = (event) => {
	const notificationEl = document.getElementById('spinner')
	notificationEl.classList.add('on')
}

const hideSpinner = (event) => {
	const notificationEl = document.getElementById('spinner')
	notificationEl.classList.remove('on')
}

const setTheme = (event) => {
	if (event && event.payload != null) {
		Theme.set(event.payload)
	}
}

const setToolbarVisibility = (event) => {
	if (event.payload) {
		document.documentElement.style.setProperty(`--toolbar-height`, '48px')
		document.getElementById('toolbar').classList.remove('hidden')
	} else {
		document.documentElement.style.setProperty(`--toolbar-height`, '0')
		document.getElementById('toolbar').classList.add('hidden')
	}
}

const enableButton = (id, event) => {
	if (event && event.payload != null) {
		if (event.payload) {
			document.getElementById(id).removeAttribute('disabled')
		} else {
			document.getElementById(id).setAttribute('disabled', 'disabled')
		}
	}
}
