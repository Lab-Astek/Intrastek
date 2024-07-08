"use client";

import { get } from "@/api/request";
import { Activity } from "@/types/activity";
import { UUID } from "crypto";
import { FC } from "react";

// type ActivityPageProps = {
//     activity: Activity;
// };

export default function Home({ params }) {
    return (
        <div>
            <h1>Activity</h1>
            <p>{params.id}</p>
            <p>Activity page content</p>
        </div>
    );
}


export async function getStaticPaths() {
    let res = await get("/activities");

    let uuids = res.data.map((uuid) => {
        console.log(uuid);
        return {
            params: {
                id: uuid,
            },
        };
    });

    return {
        uuids,
        fallback: false,
    }
}

export async function getStaticProps({ params }) {
    // let res = await get(`/activities/${params.id}`);

    return {
        props: {
            params,
        },
    };
}
