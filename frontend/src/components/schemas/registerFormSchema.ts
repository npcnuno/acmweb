import {
  email,
  minLength,
  object,
  string,
  pipe,
  regex,
  nonEmpty,
  maxLength,
  optional,
  empty,
} from "valibot";

const phoneRegex = /^((\+\d{1,3}(-| )?\(?\d\)?(-| )?\d{1,5})|(\(?\d{2,6}\)?))(-| )?(\d{3,4})(-| )?(\d{4})(( x| ext)\d{1,5}){0,1}$/;

export const registerFormSchema = object({
  name: pipe(string("Your Name must be a string"), nonEmpty("Please enter your full Name")),
  email: pipe(string("Your email must be a string"), email("The email address is badly formatted"), nonEmpty("Please enter your email")),
  student_id: pipe(string("Your student id must be a string"), nonEmpty("Please enter your student id")),
  phone: pipe(string(), nonEmpty("Please enter your phone Number"), minLength(9), maxLength(13), regex(phoneRegex, "Invalid phone number")),
  registration_info: pipe(string(), empty()),
});
