"use client";

import { get } from "@/api/request";
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

function valTranslate(value: any) {
  if (typeof value === "object") {
    if ("start" in value) {
      return `${value.start} - ${value.end}`;
    }
    return JSON.stringify(value);
  }
  return value.toString();
}

export default function Home({ params }: { params: { id: string } }) {
  let [activity, setActivity] = useState<Activity | undefined>(undefined);
  let [error, setError] = useState<boolean>(false);

  useEffect(() => {
    get(`activities/${params.id}`)
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
