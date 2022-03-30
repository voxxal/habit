import ky from "ky";
import { State } from "./state";

// post call fails with: Fetch API cannot load http://localhost:8080/sync due to access control checks
const sync = async (state: State) => ky.post("http://localhost:8080/sync", {json: state})

export {
    sync
}