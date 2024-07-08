import { ActivityType, ActivityRequest, Module } from "@/types/activity";
import { post, get } from "./request";
import { UUID } from "crypto";

function moduleFromString(module: string): Module  | undefined {
    switch (module) {
        case "Cpe":
            return Module.Cpe;
        case "Psu":
            return Module.Psu;
        case "Mul":
            return Module.Mul;
        case "Mat":
            return Module.Mat;
        case "Web":
            return Module.Web;
        case "Aia":
            return Module.Aia;
        default:
            return undefined;
    }
}

export async function createActivity(location: string, asteks: number, activityType: ActivityType, module: string) {
    let end = new Date();

    end.setHours(end.getHours() + 2);
    let data: ActivityRequest = {
        activity: activityType,
        interval: {
            start: new Date(),
            end: end
        },
        location: location,
        needed_asteks: asteks,
        module: moduleFromString(module)
    };

    return post("activities", data).then()
}

export async function getActivity(uuid: UUID) {
    return get(`activities/${uuid}`)
}
