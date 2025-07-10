<template>
    <div class="settings">
		<h2>Settings</h2> <br>

		<!-- Dropdown to select character set for ASCII art -->
		<label for="charSet">Character Set: </label>
		<select id="charSet" v-model="charSet" @change="$emit('update:char-set', charSet)">
			<option value="standard">Standard</option>
			<option value="simple">Simple</option>
			<option value="complex">Complex</option>
		</select>
		<br>

		<!-- Checkbox to enable/disable color in ASCII output -->
		<label for="colourCheck">Colour: </label>
		<input type="checkbox" id="colourCheck" v-model="isColour" @change="$emit('update:is-colour', isColour)">
		
		<!-- Slider to adjust brightness of the image -->
		<label for="brightness">Brightness: </label>
		<input type="range" id="brightness" v-model.number="brightness" min="0.1" max="3" step="0.1" @change="$emit('update:brightness', brightness)">
		<span>{{ brightness }}</span>
		
		<!-- Slider to adjust contrast of the image -->
		<label for="contrast">Contrast: </label>
		<input type="range" id="contrast" v-model.number="contrast" min="0.1" max="3" step="0.1" @change="$emit('update:contrast', contrast)">
		<span>{{ contrast }}</span>
		<br>
		
		<!-- Input for output width, triggers aspect ratio adjustment on change -->
		<label for="width">Width (10-500):</label>
		<input type="number" id="width" v-model.number="localWidth" min="10" max="500" @change="$emit('update:width', localWidth)">
		
		<!-- Input for output height, triggers aspect ratio adjustment on change -->
		<label for="height">Height (10-500):</label>
		<input type="number" id="height" v-model.number="localHeight" min="10" max="500" @change="$emit('update:height', localHeight)">
		
		<!-- Checkbox to lock/unlock aspect ratio -->
		<label for="aspectLock">Aspect Lock:</label>
		<input type="checkbox" id="aspectLock" v-model="aspectLock" @change="$emit('update:aspect-lock', aspectLock)">
		<br>
		
		<!-- Slider to adjust aspect ratio correction for character dimensions -->
		<label for="aspectCorrection">Aspect Ratio Correction:</label>
		<input type="range" id="aspectCorrection" v-model.number="aspectRatioCorrection" min="0.1" max="2" step="0.01" @change="$emit('update:aspect-ratio-correction', aspectRatioCorrection)">
		<span>{{ aspectRatioCorrection }}</span>
		<!-- Note explaining aspect ratio correction -->
		<span style="color: red">* aspect ratio correction accounts for characters being more tall then wide.</span>
	</div> 
</template>

<script lang="ts" setup>
import { ref, watch, defineProps, defineEmits } from 'vue';

const props = defineProps(['width', 'height'])
defineEmits([
    'update:char-set', 'update:width', 'update:height', 'update:aspect-lock',
    'update:is-colour', 'update:brightness', 'update:contrast', 'update:aspect-ratio-correction'
])

const charSet = ref("standard");
const localWidth = ref(props.width);
const localHeight = ref(props.height);
const isColour = ref(false);
const brightness = ref(1);
const contrast = ref(1);
const aspectLock = ref(true);
const aspectRatioCorrection = ref(0.55);

// Sync localWidth and localHeight with props when they change
watch(() => props.width, (newWidth) => {
    localWidth.value = newWidth;
});
watch(() => props.height, (newHeight) => {
    localHeight.value = newHeight;
});
</script>

<style>
.settings {
	border: 1px solid white;
	width: 100%;
	height: 100%;
}
</style>