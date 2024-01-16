const updateTagInfo = (event) => {
	if (event && event.payload != null) {
		if (event.payload.Agent != null) {
			updateAgentInfo(event.payload.Agent)

		} else if (event.payload.Egg != null) {
			updateEggInfo(event.payload.Egg)

		} else if (event.payload.GardenBox != null) {
			updateGardenBoxInfo(event.payload.GardenBox)

		} else {
			const tagInfoEl = document.getElementById('tag-info')
			tagInfoEl.innerHTML = 'Unknown tag type'
		}

		const duplicateTagButton = document.getElementById('duplicate-tag-button')
		if (duplicateTagButton != null) {
			duplicateTagButton.addEventListener('click', () => {
				tauri_invoke('duplicate_tag', {})
			})
		}

		const removeTagButton = document.getElementById('remove-tag-button')
		if (removeTagButton != null) {
			removeTagButton.addEventListener('click', () => {
				tauri_invoke('remove_tag', {})
			})
		}
	}
}

const setupPropEvent = (propName, isStrValue) => {
	const el = document.getElementById(`prop-${propName.replace(/_/g, '-')}`)
	if (el != null) {
		if (isStrValue) {
			el.addEventListener('input', () => {
				tags[selectedTag][propName] = el.value
				tauri_invoke('update_prop_str', {
					prop: propName,
					value: el.value
				})
			})
		} else {
			el.addEventListener('input', () => {
				const newValue = parseInt(el.value)
				if (isNaN(newValue) || `${newValue}` !== `${el.value}` || newValue < 0) {
					el.classList.add('invalid')
				} else {
					el.classList.remove('invalid')
					tags[selectedTag][propName] = newValue
					tauri_invoke('update_prop_int', {
						prop: propName,
						value: newValue
					})
				}
			})
		}
	}
}
