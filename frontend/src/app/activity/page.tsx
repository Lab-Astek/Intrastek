"use client";
import { get } from "@/api/request";
import ButtonWrapper from "@/components/button";
import { UUID } from "crypto";
import ListItem from '@mui/material/ListItem';
import ListItemAvatar from '@mui/material/ListItemAvatar';
import Avatar from '@mui/material/Avatar';
import FolderIcon from '@mui/icons-material/Folder';
import List from '@mui/material/List';
import { useState } from "react";

export default function Home() {
    let [activitiesIds, setActivitiesIds] = useState<UUID[]>([]);

    get("activities").then((response) => {
        setActivitiesIds(response.data);
    });

    return (
        <main className="flex min-h-screen flex-col items-center p-24">
            <h1>Activities</h1>
            {<List>
                {activitiesIds.map((id) => (
                    <ListItem>
                        <ButtonWrapper>
                        <ListItemAvatar>
                            <Avatar>
                                <FolderIcon />
                            </Avatar>
                        </ListItemAvatar>
                            <a href={`/activity/${id}`}>{id}</a>
                        </ButtonWrapper>
                    </ListItem>
                ))}
            </List>}
            <ButtonWrapper onClick={async () => {
                let ids: UUID[] = (await get("activities")).data;
                setActivitiesIds(ids);
                console.log(ids);
            }}>Refresh</ButtonWrapper>
        </main>
    );
}