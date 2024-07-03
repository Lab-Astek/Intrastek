import { UUID } from "crypto";
import request from "./request";

export async function getAstek(id: UUID) {
    return request("GET", `asteks/${id}`, {});
}

export async function getAsteks() {
    return request("GET", "asteks", {});
}

export async function createAstek(id: UUID) {
    return request("POST", "asteks", id);
}

export async function addIndisponibility(id: UUID, indisponibility: any) {
    return request("POST", `asteks/${id}`, indisponibility);
}
