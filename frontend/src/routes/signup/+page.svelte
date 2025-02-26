<script lang="ts">
	import { defaults, superForm } from 'sveltekit-superforms/client';
	import { valibot } from 'sveltekit-superforms/adapters';
	import { registerFormSchema } from '$lib/components/schemas/registerFormSchema';

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
			phone: $form.phone
		};
		console.log('Form submitted with data:', formData);

		// try {
		//
		//
		// } catch (error) {
		//   console.log(error)
		// }
		return fetch('/api/signup/', {
			method: 'POST',
			body: JSON.stringify({})
		});

		// Here you can add code to handle the form submission,
		// like sending data to a server
	}
</script>

<div class=" animated-background bg-gradient-to-trjustify-center flex items-center">
	<form class="h-6/5 relative place-self-center rounded-xl bg-gray-700 p-10" on:submit={submit}>
		<div class="py-5">
			<p class="text-center text-4xl font-bold">Register with ACM</p>
		</div>

		<label>
			Name: <br />
			<input
				name="name"
				class="focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-500 dark:focus:border-primary-500 block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-gray-900 shadow-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 sm:text-sm"
				type="text"
				aria-invalid={$errors.name ? 'true' : undefined}
				bind:value={$form.name}
				{...$constraints.name}
			/>
			{#if $errors.name}<span class="invalid">{$errors.name}</span>{/if}
		</label>

		<label>
			E-mail: <br />
			<input
				name="email"
				class="focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-500 dark:focus:border-primary-500 block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-gray-900 shadow-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 sm:text-sm"
				type="email"
				aria-invalid={$errors.email ? 'true' : undefined}
				bind:value={$form.email}
				{...$constraints.email}
			/>
			{#if $errors.email}<span class="invalid">{$errors.email}</span>{/if}
		</label>

		<label>
			Student ID: <br />
			<input
				name="student_id"
				class="focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-500 dark:focus:border-primary-500 block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-gray-900 shadow-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 sm:text-sm"
				type="text"
				aria-invalid={$errors.student_id ? 'true' : undefined}
				bind:value={$form.student_id}
				{...$constraints.student_id}
			/>
			{#if $errors.student_id}<span class="invalid">{$errors.student_id}</span>{/if}
		</label>

		<label>
			Phone Number: <br />
			<input
				name="phone"
				class="focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-500 dark:focus:border-primary-500 block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-gray-900 shadow-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 sm:text-sm"
				type="text"
				aria-invalid={$errors.phone ? 'true' : undefined}
				bind:value={$form.phone}
				{...$constraints.phone}
			/>
			{#if $errors.phone}<span class="invalid">{$errors.phone}</span>{/if}
		</label>

		<label>
			How did you find us? <br />
			<input
				name="Tell us about it ...."
				class="focus:ring-primary-500 focus:border-primary-500 dark:focus:ring-primary-500 dark:focus:border-primary-500 block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-gray-900 shadow-sm dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 sm:text-sm"
				type="text"
			/>
		</label>
		<!-- FIXME:  CAN'T validate input of optionals working in backgruond-->
		<div class="py-5"></div>
		<button
			class="bg-primary-700 hover:bg-primary-800 focus:ring-primary-300 dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800 w-full rounded-lg bg-_ACM-primary py-2.5 text-center text-sm font-medium text-white focus:ring-4"
			>Submit</button
		>
		{#if $delayed}Working...{/if}
	</form>
</div>

<style>
	.invalid {
		color: red;
	}
</style>
