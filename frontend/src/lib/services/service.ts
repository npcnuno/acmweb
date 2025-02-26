// wasmAuth.ts
// This module provides TypeScript wrappers for the wasm functions
// (admin_login, user_login, etc.) and triggers an alert on errors.

import {
  admin_login,
  user_login,
  validate_token_admin,
  validate_token_user,
  refresh_token_admin,
  refresh_token_user,
  add_student,
  get_student,
  delete_student,
  get_posts,
  get_post,
  get_projects,
  get_project,
} from "wasm-test"; // adjust the import path as needed

import { showAlert } from "../../stores/alertStore"; // adjust path as needed

import {
  authToken,
  refreshToken,
  postsStore,
  projectsStore,
  userData,
} from "../../stores/globalStores";

/**
 * Utility function to parse a JSON string returned from a wasm function.
 */
function parseWasmResponse(response: string): any {
  try {
    return JSON.parse(response);
  } catch (error) {
    console.error("Failed to parse wasm response:", response);
    throw new Error("Invalid wasm response format");
  }
}

/**
 * Helper to handle errors: show an alert and rethrow the error.
 */
function handleError(error: any, context: string) {
  const errorMessage = error?.message || "Unknown error";
  showAlert(`${context}: ${errorMessage}`, 3000);
  console.error(context, error);
  throw error;
}

/**
 * Wraps the wasm admin_login function.
 * On success, stores auth and refresh tokens globally.
 */
export async function adminLogin(
  email: string,
  password: string,
  institution: string,
): Promise<any> {
  try {
    const response = await admin_login(email, password, institution);
    const parsed = parseWasmResponse(response);
    if (parsed.code === "0") {
      // Update global authentication tokens
      authToken.set(parsed.description.auth);
      refreshToken.set(parsed.description.refresh);
    }
    return parsed;
  } catch (error) {
    handleError(error, "adminLogin failed");
  }
}

/**
 * Wraps the wasm user_login function.
 * On success, stores auth and refresh tokens globally.
 */
export async function userLogin(email: string, password: string): Promise<any> {
  try {
    const response = await user_login(email, password);
    const parsed = parseWasmResponse(response);
    if (parsed.code === "0") {
      authToken.set(parsed.description.auth);
      refreshToken.set(parsed.description.refresh);
    }
    return parsed;
  } catch (error) {
    handleError(error, "userLogin failed");
  }
}

/**
 * Wraps the wasm validate_token_admin function.
 */
export async function validateTokenAdmin(token: string): Promise<any> {
  try {
    const response = await validate_token_admin(token);
    return parseWasmResponse(response);
  } catch (error) {
    handleError(error, "validateTokenAdmin failed");
  }
}

/**
 * Wraps the wasm validate_token_user function.
 */
export async function validateTokenUser(token: string): Promise<any> {
  try {
    const response = await validate_token_user(token);
    return parseWasmResponse(response);
  } catch (error) {
    handleError(error, "validateTokenUser failed");
  }
}

/**
 * Wraps the wasm refresh_token_admin function.
 * On success, updates the global auth token.
 */
export async function refreshTokenAdmin(refreshTokenStr: string): Promise<any> {
  try {
    const response = await refresh_token_admin(refreshTokenStr);
    const parsed = parseWasmResponse(response);
    if (parsed.code === "0") {
      authToken.set(parsed.description.auth);
    }
    return parsed;
  } catch (error) {
    handleError(error, "refreshTokenAdmin failed");
  }
}

/**
 * Wraps the wasm refresh_token_user function.
 * On success, updates the global auth token.
 */
export async function refreshTokenUser(refreshTokenStr: string): Promise<any> {
  try {
    const response = await refresh_token_user(refreshTokenStr);
    const parsed = parseWasmResponse(response);
    if (parsed.code === "0") {
      authToken.set(parsed.description.auth);
    }
    return parsed;
  } catch (error) {
    handleError(error, "refreshTokenUser failed");
  }
}

/**
 * Wraps the wasm add_student function.
 */
export async function addStudent(
  token: string,
  name: string,
  email: string,
  studentId: string,
  phone: string,
  info?: string,
): Promise<any> {
  try {
    const response = await add_student(
      token,
      name,
      email,
      studentId,
      phone,
      info,
    );
    return parseWasmResponse(response);
  } catch (error) {
    handleError(error, "addStudent failed");
  }
}

/**
 * Wraps the wasm get_student function.
 * On success, stores student data globally.
 */
export async function getStudent(
  token: string,
  studentId: string,
): Promise<any> {
  try {
    const response = await get_student(token, studentId);
    const parsed = parseWasmResponse(response);
    userData.set(parsed);
    return parsed;
  } catch (error) {
    handleError(error, "getStudent failed");
  }
}

/**
 * Wraps the wasm delete_student function.
 */
export async function deleteStudent(
  token: string,
  studentId: string,
): Promise<any> {
  try {
    const response = await delete_student(token, studentId);
    return parseWasmResponse(response);
  } catch (error) {
    handleError(error, "deleteStudent failed");
  }
}

/**
 * Wraps the wasm get_posts function.
 * On success, stores posts globally.
 */
export async function getPosts(lang: string): Promise<any> {
  try {
    const response = await get_posts(lang);
    const parsed = parseWasmResponse(response);
    postsStore.set(parsed);
    return parsed;
  } catch (error) {
    handleError(error, "getPosts failed");
  }
}

/**
 * Wraps the wasm get_post function.
 */
export async function getPost(id: string): Promise<any> {
  try {
    const response = await get_post(id);
    return parseWasmResponse(response);
  } catch (error) {
    handleError(error, "getPost failed");
  }
}

/**
 * Wraps the wasm get_projects function.
 * On success, stores projects globally.
 */
export async function getProjects(lang: string): Promise<any> {
  try {
    const response = await get_projects(lang);
    const parsed = parseWasmResponse(response);
    projectsStore.set(parsed);
    return parsed;
  } catch (error) {
    handleError(error, "getProjects failed");
  }
}

/**
 * Wraps the wasm get_project function.
 */
export async function getProject(id: string): Promise<any> {
  try {
    const response = await get_project(id);
    return parseWasmResponse(response);
  } catch (error) {
    handleError(error, "getProject failed");
  }
}

/**
 * Helper function for handling gRPC errors.
 */
export function handleGrpcError(error: any): string {
  console.error("gRPC error:", error);
  return JSON.stringify({
    code: error.code ? error.code.toString() : "unknown",
    description: error.message || "Unknown error",
  });
}
