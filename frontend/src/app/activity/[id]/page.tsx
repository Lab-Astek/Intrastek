"use client";

import { Activity } from "@/types/activity";
import { useEffect, useState } from "react";
import Paper from "@mui/material/Paper";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Page from "@/components/page";
import Error from "next/error";
import { getActivity } from "@/api/activity";
import { UUID } from "crypto";
import { useAccount, useMsal } from "@azure/msal-react";

function valTranslate(value: any) {
  console.log(value);
  if (typeof value === "object" && value !== null) {
    if ("start" in value) {
      return `${value.start} - ${value.end}`;
    }
    return JSON.stringify(value);
  }
  if (value === null) return "null";
  return value.toString();
}

export default function Home({ params }: { params: { id: UUID } }) {
  let [activity, setActivity] = useState<Activity | undefined>(undefined);
  let [error, setError] = useState<boolean>(false);
  const { accounts } = useMsal();
  const user = useAccount(accounts[0] || {});

  useEffect(() => {
    getActivity(user, params.id)
      .then((response) => {
        setActivity(response.data);
      })
      .catch((error) => {
        console.error(error);
        setError(true);
      });
  }, []);

  const columns = [
    { id: "key", label: "key", minWidth: 170 },
    { id: "value", label: "value", minWidth: 100 },
  ];

  const rows = Object.entries(activity || {}).map(([key, value]) => ({
    key,
    value: valTranslate(value),
  }));

  if (!error) {
    return (
      <Page title="Activity infos">
        <Paper sx={{ width: "100%", overflow: "hidden" }}>
          <TableContainer sx={{ maxHeight: 440 }}>
            <Table stickyHeader aria-label="sticky table">
              <TableHead>
                <TableRow>
                  {columns.map((column) => (
                    <TableCell
                      key={column.id}
                      style={{ minWidth: column.minWidth }}
                    >
                      {column.label}
                    </TableCell>
                  ))}
                </TableRow>
              </TableHead>
              <TableBody>
                {rows.map((row) => {
                  return (
                    <TableRow
                      key={row.key}
                      sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
                    >
                      <TableCell component="th" scope="row">
                        {row.key}
                      </TableCell>
                      <TableCell>{row.value}</TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          </TableContainer>
        </Paper>
      </Page>
    );
  } else {
    return <Error statusCode={404}></Error>;
  }
}
