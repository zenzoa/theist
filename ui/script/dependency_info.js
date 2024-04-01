const updateDependencyInfo = (event) => {
	selectedFrames = []
	lastSelected = 0

	if (event && event.payload != null) {
		const { index, filename, text, framecount } = event.payload

		const tagInfoEl = document.getElementById('tag-info')
		tagInfoEl.innerHTML = ''

		const depInfoEl = document.createElement('div')
		depInfoEl.className = 'dependency-info'
		tagInfoEl.append(depInfoEl)

		const titleEl = document.createElement('div')
		titleEl.className = 'dependency-title'
		titleEl.innerHTML = `<span>${filename}</span>`
		depInfoEl.append(titleEl)

		const exportButton = document.createElement('button')
		exportButton.title = 'Export File'
		exportButton.innerHTML = '<img src="library/fluent/export.svg" alt="Export File">'
		exportButton.addEventListener('click', () =>
			tauri_invoke('export_dependency', { index, selectedFrames })
		)

		const contentsEl = document.createElement('div')

		if (text) {
			contentsEl.className = 'dependency-contents-text'
			contentsEl.innerHTML = text
			titleEl.append(exportButton)

		} else if (framecount) {
			const timestamp = Date.now()
			contentsEl.className = 'dependency-contents-frames'
			Array(framecount).fill(0).forEach((_, i) => {
				const frameEl = document.createElement('div')
				frameEl.id = `frame-${i}`
				frameEl.className = 'frame'
				frameEl.addEventListener('click', selectFrame.bind(this, i))
				contentsEl.append(frameEl)
				const img = document.createElement('img')
				img.src = convertFileSrc(`${timestamp}`, 'getimage') + `/${filename}/${i}`
				frameEl.append(img)
			})
			titleEl.append(exportButton)

		} else {
			contentsEl.className = 'dependency-contents-invalid'
			contentsEl.innerHTML = '<em>No preview available</em>'
		}

		depInfoEl.append(contentsEl)
	}

	hideSpinner()
}

const selectFrame = (i, event) => {
	if (event.ctrlKey) {
		if (selectedFrames.includes(i)) {
			selectedFrames = selectedFrames.filter(j => j !== i)
		} else {
			selectedFrames.push(i)
		}
		lastSelectedFrame = i

	} else if (event.shiftKey) {
		selectedFrames.push(i)
		const firstIndex = Math.min(lastSelectedFrame, i)
		const lastIndex = Math.max(lastSelectedFrame, i)
		selectedFrames = []
		for (let j = firstIndex; j <= lastIndex; j++) {
			selectedFrames.push(j)
		}

	} else if (selectedFrames.includes(i) && selectedFrames.length == 1) {
		selectedFrames = []
		lastSelectedFrame = i

	} else {
		selectedFrames = [i]
		lastSelectedFrame = i
	}

	updateSelectedFrames()
}

const selectAllFrames = () => {
	const frames = document.getElementsByClassName('frame')
	selectedFrames = Array(frames.length).fill(0).map((_, i) => i)
	updateSelectedFrames()
}

const deselectAllFrames = () => {
	selectedFrames = []
	updateSelectedFrames()
}

const updateSelectedFrames = () => {
	const frames = document.getElementsByClassName('frame')
	for (let i = 0; i < frames.length; i++) {
		if (selectedFrames.includes(i)) {
			frames[i].classList.add('selected')
		} else {
			frames[i].classList.remove('selected')
		}
	}
}
