<script lang="ts">
	import { defaults, superForm } from 'sveltekit-superforms/client';
	import { valibot } from 'sveltekit-superforms/adapters';
	import { registerFormSchema } from '../../components/schemas/registerFormSchema';

	const { form, errors, delayed, message, constraints } = superForm(
		defaults(valibot(registerFormSchema)),
		{
			SPA: true,
			validators: valibot(registerFormSchema),
			onUpdate({ form }) {
				// Form validation
			}
		}
	);

	async function submit(event: SubmitEvent) {
		event.preventDefault(); // Prevent default form submission
		const formData = {
			name: $form.name,
			email: $form.email,
			student_id: $form.student_id,
			phone: $form.phone,
			registration_info: $form.registration_info
		};
		console.log('Form submitted with data:', formData);

		// try {
		//
		//
		// } catch (error) {
		//   console.log(error)
		// }
		return;

		// Here you can add code to handle the form submission,
		// like sending data to a server
	}
</script>

<div class="flex items-center max-sm:py-20 justify-center min-h-screen">
	<form class="w-full max-w-lg bg-gray-700 p-10 rounded-xl w-11/12" on:submit={submit}>
		<div class="py-5">
			<p class="text-4xl text-center font-bold text-white">Register with ACM</p>
		</div>

		<label class="block mb-5">
			<span class="text-white">Name:</span>
			<input
				name="name"
				class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500"
				type="text"
				aria-invalid={$errors.name ? 'true' : undefined}
				bind:value={$form.name}
				{...$constraints.name}
			/>
			{#if $errors.name}
				<span class="invalid">{$errors.name}</span>
			{/if}
		</label>

		<label class="block mb-5">
			<span class="text-white">E-mail:</span>
			<input
				name="email"
				class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500"
				type="email"
				aria-invalid={$errors.email ? 'true' : undefined}
				bind:value={$form.email}
				{...$constraints.email}
			/>
			{#if $errors.email}
				<span class="invalid">{$errors.email}</span>
			{/if}
		</label>

		<label class="block mb-5">
			<span class="text-white">Student ID:</span>
			<input
				name="student_id"
				class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500"
				type="text"
				aria-invalid={$errors.student_id ? 'true' : undefined}
				bind:value={$form.student_id}
				{...$constraints.student_id}
			/>
			{#if $errors.student_id}
				<span class="invalid">{$errors.student_id}</span>
			{/if}
		</label>

		<label class="block mb-5">
			<span class="text-white">Phone Number:</span>
			<input
				name="phone"
				class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500"
				type="text"
				aria-invalid={$errors.phone ? 'true' : undefined}
				bind:value={$form.phone}
				{...$constraints.phone}
			/>
			{#if $errors.phone}
				<span class="invalid">{$errors.phone}</span>
			{/if}
		</label>

		<label class="block mb-5">
			<span class="text-white">How did you find us?</span>
			<input
				name="Tell us about it ...."
				class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500"
				type="text"
				bind:value={$form.registration_info}
			/>
		</label>

		<div class="lg:py-5"></div>
		<button
			class="text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm w-full py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800"
		>
			Submit
		</button>
		{#if $delayed}Working...{/if}
	</form>
</div>

<style>
	.invalid {
		color: red;
	}
</style>
