<script lang="ts">
	import { postLabel } from '../../models/enums';
	export let labelType: postLabel = 1;
</script>

{#if labelType === 0}
	<div class=" w-30 absolute flex items-center justify-start rounded-md bg-red-600 px-1">
		<svg
			fill="#FCA5A5"
			viewBox="0 0 64 64"
			class="h-4 w-4"
			version="1.1"
			xmlns="http://www.w3.org/2000/svg"
			xmlns:xlink="http://www.w3.org/1999/xlink"
			xml:space="preserve"
			style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
			><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
				id="SVGRepo_tracerCarrier"
				stroke-linecap="round"
				stroke-linejoin="round"
			></g><g id="SVGRepo_iconCarrier">
				<rect id="Icons" x="-704" y="-64" width="1280" height="800" style="fill:none;"></rect>
				<g id="Icons1">
					<g id="Strike"> </g> <g id="H1"> </g> <g id="H2"> </g> <g id="H3"> </g>
					<g id="list-ul"> </g> <g id="hamburger-1"> </g> <g id="hamburger-2"> </g>
					<g id="list-ol"> </g> <g id="list-task"> </g> <g id="trash"> </g>
					<g id="vertical-menu"> </g> <g id="horizontal-menu"> </g> <g id="sidebar-2"> </g>
					<g id="Pen"> </g> <g id="Pen1"> </g> <g id="clock"> </g>
					<g id="external-link"> </g> <g id="hr"> </g> <g id="info"> </g> <g id="warning"> </g>
					<path
						id="error-circle"
						d="M32.085,56.058c6.165,-0.059 12.268,-2.619 16.657,-6.966c5.213,-5.164 7.897,-12.803 6.961,-20.096c-1.605,-12.499 -11.855,-20.98 -23.772,-20.98c-9.053,0 -17.853,5.677 -21.713,13.909c-2.955,6.302 -2.96,13.911 0,20.225c3.832,8.174 12.488,13.821 21.559,13.908c0.103,0.001 0.205,0.001 0.308,0Zm-0.282,-4.003c-9.208,-0.089 -17.799,-7.227 -19.508,-16.378c-1.204,-6.452 1.07,-13.433 5.805,-18.015c5.53,-5.35 14.22,-7.143 21.445,-4.11c6.466,2.714 11.304,9.014 12.196,15.955c0.764,5.949 -1.366,12.184 -5.551,16.48c-3.672,3.767 -8.82,6.016 -14.131,6.068c-0.085,0 -0.171,0 -0.256,0Zm-12.382,-10.29l9.734,-9.734l-9.744,-9.744l2.804,-2.803l9.744,9.744l10.078,-10.078l2.808,2.807l-10.078,10.079l10.098,10.098l-2.803,2.804l-10.099,-10.099l-9.734,9.734l-2.808,-2.808Z"
					></path> <g id="plus-circle"> </g> <g id="minus-circle"> </g> <g id="vue"> </g>
					<g id="cog"> </g> <g id="logo"> </g> <g id="radio-check"> </g> <g id="eye-slash"> </g>
					<g id="eye"> </g> <g id="toggle-off"> </g> <g id="shredder"> </g>
					<g id="spinner--loading--dots-"> </g>
					<g id="react"> </g> <g id="check-selected"> </g> <g id="turn-off"> </g>
					<g id="code-block"> </g> <g id="user"> </g> <g id="coffee-bean"> </g>
					<g id="coffee-beans"> <g id="coffee-bean1"> </g> </g>
					<g id="coffee-bean-filled"> </g>
					<g id="coffee-beans-filled"> <g id="coffee-bean2"> </g> </g>
					<g id="clipboard"> </g> <g id="clipboard-paste"> </g> <g id="clipboard-copy"> </g>
					<g id="Layer1"> </g>
				</g>
			</g></svg
		>
		<span class=" px-1 text-xs text-red-300">DROPPED</span>
	</div>
{/if}
{#if labelType === 1}
	<div class=" w-30 absolute flex items-center justify-start rounded-md bg-gray-600 px-1">
		<svg
			version="1.1"
			class="h-3 w-3"
			id="Capa_1"
			xmlns="http://www.w3.org/2000/svg"
			xmlns:xlink="http://www.w3.org/1999/xlink"
			viewBox="0 0 29.334 29.334"
			xml:space="preserve"
			><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
				id="SVGRepo_tracerCarrier"
				stroke-linecap="round"
				stroke-linejoin="round"
			></g><g id="SVGRepo_iconCarrier" fill="#9CA3AF">
				<g>
					<path
						d="M14.666,0C6.578,0,0,6.58,0,14.668c0,8.087,6.578,14.666,14.666,14.666s14.668-6.579,14.668-14.666 C29.334,6.58,22.754,0,14.666,0z M14.666,25.334C8.784,25.334,4,20.549,4,14.668C4,8.785,8.784,4,14.666,4 c5.883,0,10.668,4.785,10.668,10.668C25.334,20.549,20.547,25.334,14.666,25.334z"
					></path>
				</g>
			</g></svg
		>
		<!-- UNAVAILABLE Text with background -->
		<span class=" px-1 text-xs text-gray-400">PLANNED</span>
	</div>
{/if}
{#if labelType === 2}
	<div class=" w-30 absolute flex items-center justify-start rounded-md bg-orange-500 px-1">
		<svg
			version="1.1"
			class="h-3 w-3"
			id="Capa_1"
			xmlns="http://www.w3.org/2000/svg"
			xmlns:xlink="http://www.w3.org/1999/xlink"
			viewBox="0 0 29.334 29.334"
			xml:space="preserve"
			><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
				id="SVGRepo_tracerCarrier"
				stroke-linecap="round"
				stroke-linejoin="round"
			></g><g id="SVGRepo_iconCarrier" fill="#ffc04d">
				<g>
					<path
						d="M14.666,0C6.578,0,0,6.58,0,14.668c0,8.087,6.578,14.666,14.666,14.666s14.668-6.579,14.668-14.666 C29.334,6.58,22.754,0,14.666,0z M14.666,25.334C8.784,25.334,4,20.549,4,14.668C4,8.785,8.784,4,14.666,4 c5.883,0,10.668,4.785,10.668,10.668C25.334,20.549,20.547,25.334,14.666,25.334z"
					></path>
				</g>
			</g></svg
		>
		<!-- UNAVAILABLE Text with background -->
		<span class=" px-1 text-xs text-orange-200">UNDER DEVELOPMENT</span>
	</div>
{/if}
{#if labelType === 3}
	<div class=" w-30 absolute flex items-center justify-start rounded-md bg-green-500 px-1">
		<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4"
			><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
				id="SVGRepo_tracerCarrier"
				stroke-linecap="round"
				stroke-linejoin="round"
			></g><g id="SVGRepo_iconCarrier">
				<path
					d="M9 10L12.2581 12.4436C12.6766 12.7574 13.2662 12.6957 13.6107 12.3021L20 5"
					stroke="#BBF7D0"
					stroke-width="2"
					stroke-linecap="round"
				></path>
				<path
					d="M21 12C21 13.8805 20.411 15.7137 19.3156 17.2423C18.2203 18.7709 16.6736 19.9179 14.893 20.5224C13.1123 21.1268 11.187 21.1583 9.38744 20.6125C7.58792 20.0666 6.00459 18.9707 4.85982 17.4789C3.71505 15.987 3.06635 14.174 3.00482 12.2945C2.94329 10.415 3.47203 8.56344 4.51677 6.99987C5.56152 5.4363 7.06979 4.23925 8.82975 3.57685C10.5897 2.91444 12.513 2.81996 14.3294 3.30667"
					stroke="#BBF7D0"
					stroke-width="2"
					stroke-linecap="round"
				></path>
			</g></svg
		>
		<span class=" px-1 text-xs text-green-200">DONE</span>
	</div>
{/if}

<!-- <!-- Cross Icon -->
<!-- 	<svg -->
<!-- 		viewBox="0 0 25 25" -->
<!-- 		version="1.1" -->
<!-- 		class="h-3 w-3" -->
<!-- 		xmlns="http://www.w3.org/2000/svg" -->
<!-- 		xmlns:xlink="http://www.w3.org/1999/xlink" -->
<!-- 		fill="#9CA3AF" -->
<!-- 	> -->
<!-- 		<!-- Updated fill color to match text --> -->
<!-- 		<g id="SVGRepo_bgCarrier" stroke-width="0"></g> -->
<!-- 		<g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g> -->
<!-- 		<g id="SVGRepo_iconCarrier"> -->
<!-- 			<g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"> -->
<!-- 				<g id="Icon-Set" transform="translate(-467.000000, -1039.000000)" fill="#9CA3AF"> -->
<!-- 					<!-- Same text color --> -->
<!-- 					<path -->
<!-- 						d="M489.396,1061.4 C488.614,1062.18 487.347,1062.18 486.564,1061.4 L479.484,1054.32 L472.404,1061.4 C471.622,1062.18 470.354,1062.18 469.572,1061.4 C468.79,1060.61 468.79,1059.35 469.572,1058.56 L476.652,1051.48 L469.572,1044.4 C468.79,1043.62 468.79,1042.35 469.572,1041.57 C470.354,1040.79 471.622,1040.79 472.404,1041.57 L479.484,1048.65 L486.564,1041.57 C487.347,1040.79 488.614,1040.79 489.396,1041.57 C490.179,1042.35 490.179,1043.62 489.396,1044.4 L482.316,1051.48 L489.396,1058.56 C490.179,1059.35 490.179,1060.61 489.396,1061.4 Z" -->
<!-- 						id="cross" -->
<!-- 					> -->
<!-- 					</path> -->
<!-- 				</g> -->
<!-- 			</g> -->
<!-- 		</g> -->
<!-- 	</svg> -->
