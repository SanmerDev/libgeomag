from pathlib import Path


def main():
    root_folder = Path(__file__).resolve().parent
    cof_file = root_folder.joinpath("IGRF.COF")
    cof = cof_file.read_text()

    c_g = []
    c_h = []
    c_zero = []
    for line in cof.splitlines():
        t = line.strip().split()
        v = [float(v) for v in t[3:]]

        if t[0] == "g":
            c_g.append(v)

        elif t[0] == "h":
            c_h.append(v)

            if t[1] == t[2]:
                if len(c_zero) == 0:
                    c_zero = [float(v) * 0 for v in range(len(v))]

                c_h.append(c_zero)

    c_h.insert(0, c_h.pop())

    l0 = len(c_g[0])
    l1 = len(c_g)
    print(f"const IGRF_COEFFICIENTS_G: [[f64; {l0}]; {l1}] = {c_g};")
    print(f"const IGRF_COEFFICIENTS_H: [[f64; {l0}]; {l1}] = {c_h};")


if __name__ == "__main__":
    main()
