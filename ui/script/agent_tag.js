const updateAgentInfo = (tag) => {
	tags[selectedTag] = tag

	const tagInfoEl = document.getElementById('tag-info')
	tagInfoEl.innerHTML = `
		<div class="input-row">
			<label>
				<span class="label">Agent Tag Name</span>
				<input id="prop-name" value="${tag.name}">
			</label>
			<button id="duplicate-tag-button" title="Duplicate Tag">
				<img src="library/fluent/duplicate.svg" alt="Duplicate Tag">
			</button>
			<button id="remove-tag-button" title="Remove Tag">
				<img src="library/fluent/delete.svg" alt="Remove Tag">
			</button>
		</div>
		<div class="input-row">
			<label class="dropdown-container">
				<span class="label">Game Support</span>
				<select id="prop-game-support">
					<option value="Creatures3" ${tag.game_support === 'Creatures3' ? 'selected' : ''}>Creatures 3</option>
					<option value="DockingStation" ${tag.game_support === 'DockingStation' ? 'selected' : ''}>Docking Station</option>
				</select>
				<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Animation File</span>
				<select id="prop-animation-file">
					<option value="">(none)</option>
					${dependencies.filter(f => f.extension === 'c16').map(f => {
						const filename = `${f.name}.${f.extension}`
						return `<option value="${filename}" ${tag.animation_file === filename ? 'selected' : ''}>${filename}</option>`
					}).join('')}
				</select>
				<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Animation String</span>
				<input id="prop-animation-string" value="${tag.animation_string}">
			</label>
		</div>`
	if (tag.game_support === 'DockingStation') {
		tagInfoEl.innerHTML += `
			<div class="input-row">
				<label>
					<span class="label">Sprite First Image</span>
					<input id="prop-sprite-first-image" type="number" step="1" min="0" value="${tag.sprite_first_image}">
				</label>
			</div>
			${tag.descriptions.map((description, i) =>
				`<div class="input-row tall">
					<span class="label">Description</span>
					<div class="stack fill">
						<div class="dropdown-container">
							<select id="prop-description-language-${i}">
								<option value="English" ${description.language === 'English' ? 'selected' : ''}>English</option>
								<option value="Spanish" ${description.language === 'Spanish' ? 'selected' : ''}>Spanish</option>
								<option value="French" ${description.language === 'French' ? 'selected' : ''}>French</option>
								<option value="German" ${description.language === 'German' ? 'selected' : ''}>German</option>
								<option value="Italian" ${description.language === 'Italian' ? 'selected' : ''}>Italian</option>
								<option value="Dutch" ${description.language === 'Dutch' ? 'selected' : ''}>Dutch</option>
							</select>
							<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
						</div>
						<textarea id="prop-description-text-${i}" rows=4>${description.text}</textarea>
					</div>
					<div class="stack">
						${tag.descriptions.length > 1 ?
							`<button id="remove-description-button-${i}" title="Remove Description Translation">
								<img src="library/fluent/remove.svg" alt="Remove Description Translation">
							</button>`
							: ''
						}
						<span class="fill"></span>
						${i === tag.descriptions.length - 1 ?
							`<button id="add-description-button" title="Add Description Translation">
								<img src="library/fluent/add.svg" alt="Add Description Translation">
							</button>`
							: ''
						}
					</div>
				</div>`
			).join('')}
			<div class="input-row">
				<label>
					<span class="label">Web Label</span>
					<input id="prop-web-label" value="${tag.web_label}">
				</label>
			</div>
			<div class="input-row">
				<label>
					<span class="label">Web URL</span>
					<input id="prop-web-url" value="${tag.web_url}">
				</label>
			</div>`
	} else if (tag.game_support === 'Creatures3') {
		tagInfoEl.innerHTML += `
			<div class="input-row">
				<label>
					<span class="label">Bioenergy</span>
					<input id="prop-bioenergy" type="number" step="1" min="0" value="${tag.bioenergy}">
				</label>
			</div>`
	}
	tagInfoEl.innerHTML += `
		<div class="input-row tall">
			<span class="label">Remove Script</span>
			<textarea id="prop-remove-script" rows=4>${tag.remove_script}</textarea>
			<button id="generate-remove-script-button" title="Generate From Script">
				<img src="library/fluent/reload.svg" alt="Generate From Script">
			</button>
		</div>
	`

	document.getElementById('prop-name').addEventListener('input', (event) => {
		if (event.target != null) {
			tag.name = event.target.value
			tauri_invoke('update_prop_str', {
				prop: 'name',
				value: event.target.value
			})
			tab = document.getElementById(`tag-${selectedTag}`)
			if (tab != null) {
				tab.innerText = event.target.value
				tab.title = event.target.value
			}
		}
	})

	setupPropEvent('game_support', true)
	setupPropEvent('bioenergy', false)
	setupPropEvent('web_label', true)
	setupPropEvent('web_url', true)
	setupPropEvent('animation_file', true)
	setupPropEvent('animation_string', true)
	setupPropEvent('sprite_first_image', false)
	setupPropEvent('remove_script', true)

	document.getElementById('generate-remove-script-button').addEventListener('click', () => {
		tauri_invoke('generate_remove_script', {})
	})

	tag.descriptions.forEach((description, i) => {
		const languageEl = document.getElementById(`prop-description-language-${i}`)
		if (languageEl != null) {
			languageEl.addEventListener('input', () => {
				description.language = languageEl.value
				tauri_invoke('update_description_language', {
					index: i,
					value: languageEl.value
				})
			})
		}

		const textEl = document.getElementById(`prop-description-text-${i}`)
		if (textEl != null) {
			textEl.addEventListener('input', () => {
				description.text = textEl.value
				tauri_invoke('update_description_text', {
					index: i,
					value: textEl.value
				})
			})
		}

		const removeDescriptionButton = document.getElementById(`remove-description-button-${i}`)
		if (removeDescriptionButton != null) {
			removeDescriptionButton.addEventListener('click', () => {
				tauri_invoke('remove_description', { index: i })
			})
		}
	})

	const addDescriptionButton = document.getElementById('add-description-button')
	if (addDescriptionButton != null) {
		addDescriptionButton.addEventListener('click', () => {
			tauri_invoke('add_description', {})
		})
	}
}
