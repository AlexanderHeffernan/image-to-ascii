<template>
	<!-- Main heading for the image-to-ASCII converter -->
	<h1>Convert Here</h1>
	<!-- File input to upload an image, triggers handleImgChange on file selection -->
	<input style="color: white" type="file" @change="handleImgChange" />

	<br> <br>
	<!-- Container for conversion settings -->
	<div class="settings">
		<!-- Dropdown to select character set for ASCII art -->
		<label for="charSet">Character Set: </label>
		<select id="charSet" v-model="charSet">
			<option value="standard">Standard</option>
			<option value="simple">Simple</option>
			<option value="complex">Complex</option>
		</select>
		<br>
		<!-- Checkbox to enable/disable color in ASCII output -->
		<label for="colourCheck">Colour: </label>
		<input type="checkbox" id="colourCheck" v-model="isColour">
		<!-- Slider to adjust brightness of the image -->
		<label for="brightness">Brightness: </label>
		<input type="range" id="brightness" v-model.number="brightness" min="0.1" max="3" step="0.1">
		<span>{{ brightness }}</span>
		<!-- Slider to adjust contrast of the image -->
		<label for="contrast">Contrast: </label>
		<input type="range" id="contrast" v-model.number="contrast" min="0.1" max="3" step="0.1">
		<span>{{ contrast }}</span>
		<br>
		<!-- Input for output width, triggers aspect ratio adjustment on change -->
		<label for="width">Width (10-500):</label>
		<input type="number" id="width" v-model.number="width" min="10" max="500" @change="adjustToAspectRatio(true)">
		<!-- Input for output height, triggers aspect ratio adjustment on change -->
		<label for="height">Height (10-500):</label>
		<input type="number" id="height" v-model.number="height" min="10" max="500" @change="adjustToAspectRatio(false)">
		<!-- Checkbox to lock/unlock aspect ratio -->
		<label for="aspectLock">Aspect Lock:</label>
		<input type="checkbox" id="aspectLock" v-model="aspectLock">
		<br>
		<!-- Slider to adjust aspect ratio correction for character dimensions -->
		<label for="aspectCorrection">Aspect Ratio Correction:</label>
		<input type="range" id="aspectCorrection" v-model.number="aspectRatioCorrection" min="0.1" max="2" step="0.01" @change="adjustToAspectRatio(true)">
		<span>{{ aspectRatioCorrection }}</span>
		<!-- Note explaining aspect ratio correction -->
		<span style="color: red">* aspect ratio correction accounts for characters being more tall then wide.</span>
	</div> 
	<br>

	<!-- Button to submit image and settings for conversion -->
	<button type="submit" @click="submitImgToConverter">Submit to Preview</button>

	<!-- Slider to adjust zoom (font size) of ASCII art display -->
	<label for="fontSize">Zoom: </label>
	<input type="range" id="fontSize" v-model.number="fontSize" min="2" max="30" step="1"/>
	<span>{{ fontSize }}px</span>
	<!-- Preformatted text area to display the ASCII art, with dynamic font size -->
	<pre :style="{ fontSize: `${fontSize}px` }" class="asciiArt" v-html="asciiArtHtml"></pre>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

// Reactive variables for various functionality
const uploadedImg = ref<File|null>(null);
const charSet = ref("standard");
const isColour = ref(false);
const brightness = ref(1);
const contrast = ref(1);
const width = ref(200);
const height = ref(100);
const aspectRatio = ref(0);
const aspectRatioCorrection = ref(0.55);
const aspectLock = ref(true);
const fontSize = ref(10);
const asciiArtHtml = ref<string>("");

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
	//const response = await fetch('https://192.168.68.59:8444/convert-image', {
	const response = await fetch('https://192.168.1.95:8444/convert-image', {
		method: 'POST',
		body: formData,
	});

	// Parse the ASCII grid from the server response
	const asciiGrid = await response.json();
	console.log(asciiGrid);

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