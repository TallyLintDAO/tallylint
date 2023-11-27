from ic.client import Client
from ic.identity import Identity
from ic.agent import Agent
import pem


def get_identity():
    file_path = "/home/btwl/.config/dfx/identity/btwl0/identity.pem"

    # Identity and Client are dependencies of Agent
    iden = Identity()
    client = Client()
    agent = Agent(iden, client)
    certs = pem.parse_file(file_path)

    # Get the first certificate in the list and convert it to a string
    cert = certs[0]
    pem_string = cert.as_bytes().decode("utf-8")

    # Use the pem_string as the argument for the from_pem method
    iden.from_pem(pem_string)
    return iden


identity = get_identity()

print(identity)
client = Client()
# client.
agent = Agent(identity, client)

can1 = "bkyz2-fmaaa-aaaaa-qaaaq-cai"
from ic.candid import encode, decode, Types

name = agent.query_raw(can1, "name",encode([]))
print(name)
name = agent.query_raw("gvbup-jyaaa-aaaah-qcdwa-cai", "name", encode([]))
print(name)
