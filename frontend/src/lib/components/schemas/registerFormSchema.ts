import {
	email,
	minLength,
	object,
	string,
	pipe,
	regex,
	nonEmpty,
	maxLength,
	optional
} from 'valibot';

export const registerFormSchema = object({
	name: pipe(string('Your Name must be a string'), nonEmpty('Please enter your full Name')),
	email: pipe(
		string('Your email must be a string'),
		email('The email address is badly formatted'),
		nonEmpty('Please enter your email')
	),
	student_id: pipe(
		string('Your student id must be a string'),
		nonEmpty('Please enter your student id')
	),
	phone: pipe(
		string(),
		nonEmpty('Please enter your phone Number'),
		minLength(9),
		maxLength(13),
		regex(
			/^\+(\d{1,4})[^\d]*(\d{1,4})?[^\d]*(\d{1,4})?[^\d]*(\d{1,4})?[^\d]*(\d{1,4})?$/,
			'Invalid phone number'
		)
	),
	registration_info: optional(string())
});
