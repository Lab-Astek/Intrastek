"use client";
import { get } from "@/api/request";
import ButtonWrapper from "@/components/button";
import { UUID } from "crypto";
import ListItem from "@mui/material/ListItem";
import ListItemAvatar from "@mui/material/ListItemAvatar";
import Avatar from "@mui/material/Avatar";
import FolderIcon from "@mui/icons-material/Folder";
import List from "@mui/material/List";
import { useEffect, useState } from "react";
import Page from "@/components/page";
import { Activity } from "@/types/activity";

export default function Home() {
  let [activities, setActivities] = useState<Activity[]>([]);

  useEffect(() => {
    get("activities").then((response) => {
      setActivities(response.data);
    });
  }, []);

  return (
    <Page title={"Activities listing"}>
      <h1>Activities</h1>
      {
        <List>
          {activities.map((activity) => (
            <ListItem>
              <ButtonWrapper>
                <ListItemAvatar>
                  <Avatar>
                    <FolderIcon />
                  </Avatar>
                </ListItemAvatar>
                <a href={`/activity/${activity.id}`}>{activity.name}</a>
              </ButtonWrapper>
            </ListItem>
          ))}
        </List>
      }
      <ButtonWrapper
        onClick={async () => {
          get("activities").then((response) => {
            setActivities(response.data);
          });
        }}
      >
        Refresh
      </ButtonWrapper>
      <ButtonWrapper>
        <a href="/activity/create">Create an activity</a>
      </ButtonWrapper>
    </Page>
  );
}
