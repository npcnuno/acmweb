import {  email, minLength,  object, string } from "valibot";

export const _loginSchema = object({
   
    email: string([
      minLength(1, "Please enter the student institutional email."),
      email("The email address is badly formatted."),
    ]),
    password: string([minLength(1, "Please Enter a Password")]),
    instutuition: string([minLength(1, "Please Enter your Instituition")])
  
  });
