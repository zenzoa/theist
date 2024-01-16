const updateGardenBoxInfo = (tag) => {
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
			<label>
				<span class="label">Category</span>
				<select id="prop-category">
					<option value=1 ${tag.category === 1 ? 'selected' : ''}>1 - Patch Plant</option>
					<option value=2 ${tag.category === 2 ? 'selected' : ''}>2 - Traditional Plant</option>
					<option value=3 ${tag.category === 3 ? 'selected' : ''}>3 - Animal</option>
					<option value=4 ${tag.category === 4 ? 'selected' : ''}>4 - Aquatic Plant</option>
					<option value=5 ${tag.category === 5 ? 'selected' : ''}>5 - Aquatic Animal</option>
					<option value=6 ${tag.category === 6 ? 'selected' : ''}>6 - Decoration</option>
					<option value=7 ${tag.category === 7 ? 'selected' : ''}>7 - Tools</option>
					<option value=8 ${tag.category === 8 ? 'selected' : ''}>8 - Misc/Other</option>
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
				<span class="label">Sprite First Image</span>
				<input id="prop-sprite-first-image" type="number" step="1" min="0" value="${tag.sprite_first_image}">
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Description</span>
				<input id="prop-description" value="${tag.description}">
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Author</span>
				<input id="prop-author" value="${tag.author}">
			</label>
		</div>
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

	setupPropEvent('category', false)
	setupPropEvent('animation_file', true)
	setupPropEvent('sprite_first_image', false)
	setupPropEvent('description', true)
	setupPropEvent('author', true)
	setupPropEvent('remove_script', true)

	document.getElementById('generate-remove-script-button').addEventListener('click', () => {
		tauri_invoke('generate_remove_script', {})
	})
}
