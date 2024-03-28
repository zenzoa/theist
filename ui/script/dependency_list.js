const updateDependencyList = (event) => {
	if (event && event.payload != null) {
		dependencies = event.payload
		checkedDependencies = []

		const el = document.getElementById('dependency-list')
		el.innerHTML = ''
		dependencies.forEach((dependency, i) => {
			const dependencyDiv = document.createElement('div')
			dependencyDiv.id = `dependency-${i}`
			dependencyDiv.className = 'dependency-item'

			const dependencyCheckbox = document.createElement('button')
			dependencyCheckbox.className = 'dependency-checkbox'
			dependencyCheckbox.title = 'Dependency for current tag'
			dependencyCheckbox.addEventListener('click', checkDependency.bind(this, i))
			dependencyDiv.append(dependencyCheckbox)

			const dependencySelect = document.createElement('button')
			dependencySelect.innerText = `${dependency.name}.${dependency.extension}`
			dependencySelect.title = `${dependency.name}.${dependency.extension}`
			dependencySelect.className = 'text-button'
			dependencySelect.addEventListener('click', selectDependency.bind(this, i))
			dependencyDiv.append(dependencySelect)

			el.append(dependencyDiv)

			if (dependency.is_checked) {
				checkedDependencies.push(i)
			}
		})

		updateDropdownLists()

		lastSelectedDependency = 0
		selectedDependencies = []
		updateSelectedDependencies()
		updateCheckedDependencies()
	}
}

const updateDropdownLists = () => {
	if (tags[selectedTag] != null) {
		const tag = tags[selectedTag]

		const animationFileList = document.getElementById('prop-animation-file')
		if (animationFileList != null) {
			animationFileList.innerHTML = generateDropdownList('c16', tag.animation_file)
		}

		const maleSpriteList = document.getElementById('prop-sprite-file-male')
		if (maleSpriteList != null) {
			maleSpriteList.innerHTML = generateDropdownList('c16', tag.sprite_file_male)
		}

		const femaleSpriteList = document.getElementById('prop-sprite-file-female')
		if (femaleSpriteList != null) {
			femaleSpriteList.innerHTML = generateDropdownList('c16', tag.sprite_file_female)
		}

		const geneticsFileList = document.getElementById('prop-genetics-file')
		if (geneticsFileList != null) {
			geneticsFileList.innerHTML = generateDropdownList('gen', tag.genetics_file)
		}

		const motherGeneticsFileList = document.getElementById('prop-genetics-file-mother')
		if (motherGeneticsFileList != null) {
			motherGeneticsFileList.innerHTML = generateDropdownList('gen', tag.genetics_file_mother)
		}

		const fatherGeneticsFileList = document.getElementById('prop-genetics-file-father')
		if (fatherGeneticsFileList != null) {
			fatherGeneticsFileList.innerHTML = generateDropdownList('gen', tag.genetics_file_father)
		}
	}
}

const generateDropdownList = (extension, prop_value) => {
	return `<option value="">(none)</option>\n` +
		dependencies
			.filter(dependency => dependency.extension === extension)
			.map(dependency => {
				const filename = `${dependency.name}.${dependency.extension}`
				const selected = (prop_value === filename) ? 'selected' : ''
				return `<option value="${filename}" ${selected}>${filename}</option>`
			})
			.join('')
}

const checkDependency = (i, event) => {
	if (selectedDependencies.includes(i)) {
		let checkedState = false
		if (checkedDependencies.includes(i)) {
			checkedDependencies = checkedDependencies.filter(f => f !== i)
		} else {
			checkedDependencies.push(i)
			checkedState = true
		}
		selectedDependencies.forEach(s => {
			if (checkedState && !checkedDependencies.includes(s)) {
				checkedDependencies.push(s)
			} else if (!checkedState && checkedDependencies.includes(s)) {
				checkedDependencies = checkedDependencies.filter(f => f !== s)
			}
		})
	} else {
		if (!checkedDependencies.includes(i)) {
			checkedDependencies.push(i)
		}
		selectedDependencies = []
		tauri_invoke('deselect_dependency')
	}

	updateSelectedDependencies()
	updateCheckedDependencies()

	tauri_invoke('check_dependency', { checkedDependencies })
}

const selectDependency = (i, event) => {
	if (event.shiftKey) {
		selectedDependencies = []
		for (let j = Math.min(i, lastSelectedDependency); j <= Math.max(i, lastSelectedDependency); j++) {
			selectedDependencies.push(j)
		}
	} else if (event.ctrlKey) {
		if (selectedDependencies.includes(i)) {
			selectedDependencies = selectedDependencies.filter(s => s !== i)
		} else {
			selectedDependencies.push(i)
		}
		lastSelectedDependency = i
	} else {
		lastSelectedDependency = i
		if (selectedDependencies.includes(i)) {
			selectedDependencies = []
			tauri_invoke('deselect_dependency')
		} else {
			selectedDependencies = [i]
			const ext = dependencies[i].extension
			if (ext === 'c16' || ext === 's16' || ext === 'blk') {
				showSpinner()
			}
			setTimeout(() => {
				tauri_invoke('select_dependency', { selectedDependency: i })
			}, 100)

		}
	}

	updateSelectedDependencies()
}

const selectAllDependencies = () => {
	selectedDependencies = dependencies.map((_, i) => i)
	updateSelectedDependencies()
}

const deselectAllDependencies = () => {
	selectedDependencies = []
	updateSelectedDependencies()
	tauri_invoke('deselect_dependency')
}

const updateCheckedDependencies = (event) => {
	if (event && event.payload != null) {
		checkedDependencies = event.payload
	}
	const dependencyCheckboxes = document.getElementsByClassName('dependency-checkbox')
	for (let i = 0; i < dependencyCheckboxes.length; i++) {
		if (checkedDependencies.includes(i)) {
			dependencyCheckboxes[i].innerHTML = `<img src="library/fluent/checkbox-checked.svg" alt="checked">`
		} else {
			dependencyCheckboxes[i].innerHTML = `<img src="library/fluent/checkbox-unchecked.svg" alt="unchecked">`
		}
	}
}

const updateSelectedDependencies = () => {
	const dependencyItems = document.getElementsByClassName('dependency-item')
	for (let i = 0; i < dependencyItems.length; i++) {
		if (selectedDependencies.includes(i)) {
			dependencyItems[i].classList.add('selected')
		} else {
			dependencyItems[i].classList.remove('selected')
		}
	}
	if (selectedDependencies.length > 0) {
		document.getElementById('extract-dependency-button').removeAttribute('disabled')
		document.getElementById('reload-dependency-button').removeAttribute('disabled')
		document.getElementById('remove-dependency-button').removeAttribute('disabled')
	} else {
		document.getElementById('extract-dependency-button').setAttribute('disabled', 'disabled')
		document.getElementById('reload-dependency-button').setAttribute('disabled', 'disabled')
		document.getElementById('remove-dependency-button').setAttribute('disabled', 'disabled')
	}
}
