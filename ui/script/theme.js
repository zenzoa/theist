class Theme {
	static dark = {
		"main-bg": "#444444",
		"toolbar-bg": "#292929",
		"sidebar-bg": "#363636",

		"divider-color": "#444444",
		"divider-color2": "#636363",

		"button-bg": "#393939",
		"button-hover-bg": "#595959",
		"button-shadow": "#222222",
		"button-active-shadow": "#444444",

		"selected-bg": "#292929",
		"selected-hover-bg": "#191919",

		"input-bg": "#595959",
		"input-disabled-bg": "#434343",
		"input-shadow": "#494949",

		"focus-outline": "#939393",

		"frame-bg": "#292929",
		"frame-outline": "#ffffff",

		"text-color": "#ffffff",
		"icon-filter": "invert(1)",
	}

	static light = {
		"main-bg": "#dddddd",
		"toolbar-bg": "#b0b0b0",
		"sidebar-bg": "#c9c9c9",

		"divider-color": "#9d9d9d",
		"divider-color2": "#9d9d9d",

		"button-bg": "#c3c3c3",
		"button-hover-bg": "#a0a0a0",
		"button-shadow": "#8b8b8b",
		"button-active-shadow": "#797979",

		"selected-bg": "#b0b0b0",
		"selected-hover-bg": "#a0a0a0",

		"input-bg": "#f3f3f3",
		"input-disabled-bg": "#e0e0e0",
		"input-shadow": "#e0e0e0",

		"focus-outline": "#9d9d9d",

		"frame-bg": "#b0b0b0",
		"frame-outline": "#222222",

		"text-color": "#222222",
		"icon-filter": "invert(0.2)"
	}

	static purple = {
		"main-bg": "#3F3A63",
		"toolbar-bg": "#201E35",
		"sidebar-bg": "#2f2b50",

		"divider-color": "#3F3A63",
		"divider-color2": "#615e81",

		"button-bg": "#34305b",
		"button-hover-bg": "#504c72",
		"button-shadow": "#201E35",
		"button-active-shadow": "#353155",

		"selected-bg": "#201E35",
		"selected-hover-bg": "#100E25",

		"input-bg": "#4B4572",
		"input-disabled-bg": "#3F3A63",
		"input-shadow": "#3F3A63",

		"focus-outline": "#737190",

		"frame-bg": "#201E35",
		"frame-outline": "#ffffff",

		"text-color": "#eaeaef",
		"icon-filter": "invert(1)"
	}

	static set(themeName) {
		const theme = Theme[themeName]
		if (theme) {
			const style = document.documentElement.style
			for (const key in theme) {
				if (theme[key]) {
					style.setProperty(`--${key}`, theme[key])
				}
			}
		}
	}
}
