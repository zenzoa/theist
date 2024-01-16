const updateEggInfo = (tag) => {
	tags[selectedTag] = tag

	const tagInfoEl = document.getElementById('tag-info')
	tagInfoEl.innerHTML = `
		<div class="input-row">
			<label>
				<span class="label">Egg Tag Name</span>
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
				<span class="label">Genetics File</span>
				<select id="prop-genetics-file">
					<option value="">(none)</option>
					${dependencies.filter(f => f.extension === 'gen').map(f => {
						const filename = `${f.name}.${f.extension}`
						return `<option value="${filename}" ${tag.genetics_file === filename ? 'selected' : ''}>${filename}</option>`
					}).join('')}
				</select>
				<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Father Genetics File</span>
				<select id="prop-genetics-file-father">
					<option value="">(none)</option>
					${dependencies.filter(f => f.extension === 'gen').map(f => {
						const filename = `${f.name}.${f.extension}`
						return `<option value="${filename}" ${tag.genetics_file_father === filename ? 'selected' : ''}>${filename}</option>`
					}).join('')}
				</select>
				<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Mother Genetics File</span>
				<select id="prop-genetics-file-mother">
					<option value="">(none)</option>
					${dependencies.filter(f => f.extension === 'gen').map(f => {
						const filename = `${f.name}.${f.extension}`
						return `<option value="${filename}" ${tag.genetics_file_mother === filename ? 'selected' : ''}>${filename}</option>`
					}).join('')}
				</select>
				<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Male Preview</span>
				<select id="prop-sprite-file-male">
					<option value="">(none)</option>
					${dependencies.filter(f => f.extension === 'c16').map(f => {
						const filename = `${f.name}.${f.extension}`
						return `<option value="${filename}" ${tag.sprite_file_male === filename ? 'selected' : ''}>${filename}</option>`
					}).join('')}
				</select>
				<div class="dropdown-arrow"><img src="library/fluent/chevron-down.svg"></div>
			</label>
		</div>
		<div class="input-row">
			<label>
				<span class="label">Female Preview</span>
				<select id="prop-sprite-file-female">
					<option value="">(none)</option>
					${dependencies.filter(f => f.extension === 'c16').map(f => {
						const filename = `${f.name}.${f.extension}`
						return `<option value="${filename}" ${tag.sprite_file_female === filename ? 'selected' : ''}>${filename}</option>`
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

	setupPropEvent('genetics_file', true)
	setupPropEvent('genetics_file_father', true)
	setupPropEvent('genetics_file_mother', true)
	setupPropEvent('sprite_file_male', true)
	setupPropEvent('sprite_file_female', true)
	setupPropEvent('animation_string', true)
}
