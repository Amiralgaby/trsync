import json
import requests
from tests.fixtures.base import TRACIM_URL
from tests.fixtures.model import User, Workspace


SETS = {
    "Set1": [
        "/file_2.txt",
        "/folder_1",
        "/folder_1/file_1.txt",
    ]
}

FILE_CONTENTS = {
    "/file_2.txt": b"Hello world !",
    "/folder_1/file_1.txt": b"Hello world again !",
}


def create_file(
    user: User,
    workspace: Workspace,
    name: str,
    content: bytes,
    parent_id: int = None,
) -> int:
    data = {}
    if parent_id is not None:
        data["parent_id"] = parent_id
    response = requests.post(
        f"http://{TRACIM_URL}/api/workspaces/{workspace.id}/files",
        files={"files": (name, content)},
        data=data,
        auth=(user.username, user.password),
    )
    assert response.status_code == 200
    response_json = json.loads(response.content)
    return response_json["content_id"]


def create_folder(
    user: User,
    workspace: Workspace,
    name: str,
    parent_id: int = None,
) -> int:
    json_ = {"label": name, "content_type": "folder"}
    if parent_id is not None:
        json_["parent_id"] = parent_id
    response = requests.post(
        f"http://{TRACIM_URL}/api/workspaces/{workspace.id}/contents",
        json=json_,
        auth=(user.username, user.password),
    )
    assert response.status_code == 200
    response_json = json.loads(response.content)
    return response_json["content_id"]


def create_set_on_remote(user: User, workspace: Workspace, set_name: str) -> None:
    content_ids = {}
    for file_path in SETS[set_name]:
        # Create only the last part (set must be ordered correctly)
        splitted = file_path[1:].split("/")
        concerned_part = splitted[-1]
        parent_id = None

        if len(splitted) > 1:
            parent_id = content_ids["/" + "/".join(splitted[:-1])]

        if concerned_part.startswith("file_"):
            id = create_file(
                user,
                workspace,
                concerned_part,
                content=FILE_CONTENTS[file_path],
                parent_id=parent_id,
            )
        elif concerned_part.startswith("folder_"):
            id = create_folder(user, workspace, concerned_part, parent_id=parent_id)

        content_ids[file_path] = id
