"""Nomos Python SDK."""

__version__ = "0.5.0"


class NomosClient:
    def __init__(self, endpoint: str):
        self.endpoint = endpoint

    async def submit_proof(self, payload: bytes, kind: str) -> str:
        raise NotImplementedError("not yet implemented")

    async def get_status(self, proof_id: str) -> dict:
        raise NotImplementedError("not yet implemented")
