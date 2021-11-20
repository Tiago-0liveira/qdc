import os 
import sys
import toml

def yn(question, default="y", options=["y","n"]):
	i = input(f"\n{question} ({options[0]}/{options[1]}) [{default}]: ").lower().strip()
	print(i, [default, ""], i in [default, ""])
	return True if i in [default, ""] else False

def real_path(*msg):
	return os.path.join(os.path.dirname(os.path.realpath(__file__)), *msg)


dir_is_qdc = real_path().split("\\")[-1:][0].lower() == "qdc"
isQdcDir = False

if os.path.isfile(real_path("Cargo.toml")) and dir_is_qdc:
	with open(real_path("Cargo.toml")) as f:
		cargo = toml.load(f)
		if "package" in cargo:
			if "qdc" == cargo["package"]["name"].lower():
				isQdcDir = True
			else:
				print("\nThis is not the QDC rust directory.\n")
				sys.exit(0)
		else:
			print("\nThis is not the QDC rust directory.\n")
			sys.exit(0)
else:
	print("\nThis is not the QDC rust directory.\n")
	sys.exit()

target = "debug"
qdc_bin_paths = [real_path("target", "debug", "qdc.exe"), real_path("target", "release", "qdc.exe")]
if "-t=debug" in sys.argv or "--target=debug" in sys.argv:
	qdc_bin_path = qdc_bin_paths[0]
elif "-t=release" in sys.argv or "--target=release" in sys.argv:
	qdc_bin_path = qdc_bin_paths[1]
	target = "release"
else:
	if not yn("\nWich target do you want to use?", default="debug", options=["debug", "release"]):
		target = "release"
		qdc_bin_path = qdc_bin_paths[1]
	qdc_bin_path = qdc_bin_paths[0 if target == "debug" else 1]

qdc_bin_found = os.path.isfile(qdc_bin_path)

if "--build" in sys.argv or "-b" in sys.argv:
	os.system(f"cargo build {f'--{target}' if target == 'release' else ''}")
elif "--no-build" in sys.argv or "-nb" in sys.argv:
	if not qdc_bin_found:
		print("\nCould not find the qdc binary.\n")
		if yn(f"\nDo you want to build({target}) now ?"):
			os.system(f"cargo build {f'--{target}' if target == 'release' else ''}")
		else:
			sys.exit(0)
else:
	if "-y" in sys.argv or "--yes" in sys.argv or yn(f"\nDo you want to build({target}) now ?"):
		os.system(f"cargo build {f'--{target}' if target == 'release' else ''}")

global_tests = [
	"", "adw", "-sadswa", "--asd", "ow -w sad"
]

def t(x):
	print(f"\n{' '*9}«Test: 'qdc.exe {x}'»")
	os.system(f"{qdc_bin_path} {x}")

#print(f"\n\n{'*'*12}Global Tests{'*'*12}\n")
#for test in global_tests: t(test)

for i,x in enumerate(["n", "new","h","help","s","shortcuts","e","edit","d","delete"]):
	for y in [x, f"--{x}" if (i+1) % 2 == 0 else f"-{x}"]:
		for z in ["", "awd", "aawd awd", "Eawdawd awd awd", "asdawd awd awd aw"]:
			t(f"{y} {z}")

