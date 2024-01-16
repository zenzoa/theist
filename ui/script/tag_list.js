const updateTagList = (event) => {
	if (event && event.payload != null) {
		selectedTag = event.payload[0]
		tags = event.payload[1]

		const tagList = document.getElementById('tag-list')

		const tagItems = document.getElementsByClassName('tag-item')
		for (let i = tagItems.length - 1; i >= 0; i--) {
			tagItems[i].remove()
		}

		const tagSep = document.getElementById('tag-list-separator')

		event.payload[1].forEach((tag, i) => {
			const tagItem = document.createElement('button')
			tagItem.id = `tag-${i}`
			tagItem.className = 'tag-item text-button' + (i === selectedTag ? ' on' : '')
			if (tag.Agent != null) {
				tagItem.innerText = tag.Agent.name
				tagItem.title = tag.Agent.name
			} else if (tag.Egg != null) {
				tagItem.innerText = tag.Egg.name
				tagItem.title = tag.Egg.name
			} else if (tag.GardenBox != null) {
				tagItem.innerText = tag.GardenBox.name
				tagItem.title = tag.GardenBox.name
			} else if (tag.Generic != null) {
				tagItem.innerText = tag.Generic.name
				tagItem.title = tag.Generic.name
			}
			tagItem.addEventListener('click', selectTag.bind(this, i))
			tagList.insertBefore(tagItem, tagSep)
			if (i === selectedTag) {
				updateTagInfo({ payload: tag })
			}
		})

		if (tags[selectedTag] == null) {
			const tagInfoEl = document.getElementById('tag-info')
			tagInfoEl.innerHTML = ''
		}
	}
}

const selectTag = (tagIndex) => {
	selectedTag = tagIndex

	const tagItems = document.getElementsByClassName('tag-item')
	for (let i = 0; i < tagItems.length; i++) {
		if (i === selectedTag) {
			tagItems[i].classList.add('on')
		} else {
			tagItems[i].classList.remove('on')
		}
	}

	tauri_invoke('select_tag', { selectedTag })
}
