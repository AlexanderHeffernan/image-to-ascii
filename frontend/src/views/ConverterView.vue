<template>
	<!-- Main heading for the image-to-ASCII converter -->
	<h1>Convert Here</h1>

	<div class="convertion-content">
		<div class="settings-drawer">
			<!-- Container for conversion settings -->
			<SettingsDrawer 
				:width = "width"
				:height = "height"
				
				@update:char-set="charSet = $event"
				@update:width="value => handleSettingChange('width', value)"
				@update:height="value => handleSettingChange('height', value)"
				@update:aspect-lock="aspectLock = $event"
				@update:is-colour="isColour = $event"
				@update:brightness="brightness = $event"
				@update:contrast="contrast = $event"
				@update:aspect-ratio-correction="value => handleSettingChange('aspectRatioCorrection', value)"
			/> 
		</div>

		<div class="image-view">
			<!-- File input to upload an image, triggers handleImgChange on file selection -->
			<input style="color: white" type="file" @change="handleImgChange" />

			<!-- Button to submit image and settings for conversion -->
			<button type="submit" @click="submitImgToConverter">Convert</button>
			<!-- Slider to adjust zoom (font size) of ASCII art display -->
			<label for="fontSize">Zoom: </label>
			<input type="range" id="fontSize" v-model.number="fontSize" min="2" max="30" step="1"/>
			<span>{{ fontSize }}px</span>
			<!-- Preformatted text area to display the ASCII art, with dynamic font size -->
			<pre :style="{ fontSize: `${fontSize}px` }" class="asciiArt" v-html="asciiArtHtml"></pre>
		</div>
	</div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import SettingsDrawer from '@/components/SettingsDrawer.vue'

// Reactive variables for various functionality
const uploadedImg = ref<File|null>(null);
const aspectRatio = ref(0);
const fontSize = ref(10);
const asciiArtHtml = ref<string>("");

const charSet = ref("standard");
const width = ref(200);
const height = ref(100);
const aspectLock = ref(true);
const isColour = ref(false);
const brightness = ref(1);
const contrast = ref(1);
const aspectRatioCorrection = ref(0.55);

// Handles image file selection and calculates aspect ratio of the selected image
const handleImgChange = (event: Event) => {
	const files = (event.target as HTMLInputElement).files;
	let isLandscape = true;
	if (files && files.length > 0) {
		uploadedImg.value = files[0]; // Get the selected image file

		// Get aspect ratio and whether it is landscape
		const img = new Image();
		const objectUrl = URL.createObjectURL(files[0]);
		img.src = objectUrl;
		img.onload = () => {
			aspectRatio.value = img.naturalWidth / img.naturalHeight;
			isLandscape = img.naturalWidth >= img.naturalHeight
			URL.revokeObjectURL(objectUrl);

			// Reset size values and adjust to aspect ratio
			width.value = 200;
			height.value = 100;
			// Adjust the smaller dimension to prepare for extremely large ratios
			adjustToAspectRatio(isLandscape)
		};
		img.onerror = () => {
			// Clean up object URL on error
			URL.revokeObjectURL(objectUrl);
		};
	}
};

const handleSettingChange = (setting: string, value: number) => {
	if (setting === 'height') {
		height.value = value;
		adjustToAspectRatio(false);
	}
	else if (setting === 'width') {
		width.value = value;
	}
	else if (setting === 'aspectRatioCorrection') {
		aspectRatioCorrection.value = value
	}
	adjustToAspectRatio(true)
}

// Adjusts width or height to maintain aspect ratio if aspectLock is enabled
const adjustToAspectRatio = (widthChange: boolean) => {
	if (!aspectLock.value) { return; }

	if (widthChange) {
		// Adjust height based on width, aspect ratio, and character height correction
		height.value = Math.round((width.value / aspectRatio.value) * aspectRatioCorrection.value);
	}
	else {
		// Adjust width based on height, aspect ratio, and character height correction
		width.value = Math.round((height.value * aspectRatio.value) / aspectRatioCorrection.value);
	}
}

// Submits the image and settings to the server for ASCII conversion
const submitImgToConverter = async () => {
	if (!uploadedImg.value) {
		alert("Please first upload an image.");
		return;
	}

	// Simple implementation of multiple character sets
	let chars = "";
	if (charSet.value === "standard") {
		chars = " .:-=+*#%@";
	}
	else if (charSet.value === "simple") {
		chars = " .-*#@"
	}
	else if (charSet.value === "complex") {
		chars = " .ʼ`^,:;Iil!i><~+_-?][}{1)(|/tfrjxnuvcxzYXUJCLQ0OZmwpqbdkhao*#MW&8%B@$"
	}

	// Create FormData to send image and configuration to server
	const formData = new FormData();
	formData.append('image', uploadedImg.value);
	formData.append('config', JSON.stringify({ 
		character_set: chars.split(''), // Convert string to array of characters
		output_width: width.value, 
		output_height: height.value, 
		brightness_factor: brightness.value, 
		contrast_factor: contrast.value, 
		is_color: isColour.value, 
		aspect_ratio_correction: aspectRatioCorrection.value 
	}));
	
	// Send POST request to conversion endpoint
	const response = await fetch('https://192.168.68.58:8444/convert-image', {
	//const response = await fetch('https://192.168.1.95:8444/convert-image', {
		method: 'POST',
		body: formData,
	});

	// Parse the ASCII grid from the server response
	const asciiGrid = await response.json();

	// Convert ASCII grid to HTML, applying color if enabled
	asciiArtHtml.value = asciiGrid.map((row: any[]) => 
		row.map(pixel => {
			if (isColour.value && pixel.rgb) {
				const [r, g, b] = pixel.rgb;
				// Wrap colored pixels in a span with RGB color styling
				return `<span style="color: rgb(${r},${g},${b})">${pixel.ch}</span>`;
			} else {
				// Return character without color
				return pixel.ch;
			}
		}).join('') // Join characters in a row
	).join('<br>'); // Join rows with line breaks
}
</script>

<style>
h1 {
	padding-bottom: 20px;
	padding-top: 20px
}

.convertion-content {
	display: flex;
	flex-direction: row;
	width: 100%;
}

.settings-drawer {
	width: 200px;
	height: 100%;
	text-align: center;
}

.image-view {
	width: calc(100% - 200px);
	height: 100%;
	text-align: center;
}

/* Basic styling to ensure form elements have readable text color */
button {
	color: black;
}
input {
	color: black;
}
select {
	color: black;
}
option {
	color: black;
}
</style>