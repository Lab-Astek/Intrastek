import { UUID } from "crypto";
import { Interval } from "./interval";

export type Activity = {
    id: UUID,
    activity: ActivitiyType,
    interval: Interval,
    location: string,
    needed_asteks: number,
    module?: Module,
    asteks: UUID[]
};

export enum ActivitiyType {
    FollowUp = "FollowUp",
    Bootstrap = "Bootstrap",
    Review = "Review",
    Keynote = "Keynote",
    Surveillance = "Surveillance",
    Permanence = "Permanence",
}

export enum Module {
    Cpe = "Cpe",
    Psu = "Psu",
    Mul = "Mul",
    Mat = "Mat",
    Web = "Web",
    Aia = "Aia",
}

export type ActivityRequest = {
    activity: ActivitiyType,
    interval: Interval,
    location: string,
    needed_asteks: number,
    module?: Module,
};
