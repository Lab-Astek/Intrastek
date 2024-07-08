"use client";

import { get } from "@/api/request";
import { Activity } from "@/types/activity";
import { useEffect, useState } from "react";

export default function Home({ params }: { params: { id: string } }) {
    let [activity, setActivity] = useState<Activity | undefined>(undefined);

    useEffect(() => {
        get(`activities/${params.id}`).then((response) => {
            setActivity(response.data);
        });
    })

    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
            <h1>{JSON.stringify(activity)}</h1>
        </main>
    );
}