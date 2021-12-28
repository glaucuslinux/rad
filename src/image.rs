use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use super::constants;

pub fn radula_behave_bootstrap_cross_image() {
    // A custom rsync function to prevent permission errors (`-S` is not used)
    let radula_behave_rsync = |x: &str, y: &str| {
        Command::new(constants::RADULA_TOOTH_RSYNC)
            .args(&["-vaHAXx", x, y, "--delete"])
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    };

    let x = &String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap())
            .join(constants::RADULA_FILE_GLAUCUS_IMAGE)
            .to_str()
            .unwrap(),
    );

    // Create a new image
    fs::remove_file(x);
    Command::new(constants::RADULA_TOOTH_QEMU_IMAGE)
        .args(&[
            "create",
            "-f",
            "raw",
            x,
            constants::RADULA_FILE_GLAUCUS_IMAGE_SIZE,
        ])
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    // Write mbr.bin (from SYSLINUX) to the first 440 bytes of the image
    Command::new(constants::RADULA_TOOTH_DD)
        .args(&[
            &format!(
                "if={}",
                Path::new(constants::RADULA_PATH_CLUSTERS)
                    .join(constants::RADULA_FILE_SYSLINUX_MBR_BIN)
                    .to_str()
                    .unwrap()
            ),
            &format!("of={}", x),
            "conv=notrunc",
            "bs=440",
            "count=1",
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Partition the image
    Command::new(constants::RADULA_TOOTH_PARTED)
        .args(&[constants::RADULA_TOOTH_PARTED_FLAGS, x, "mklabel", "msdos"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_PARTED)
        .args(&[
            constants::RADULA_TOOTH_PARTED_FLAGS,
            "-a",
            "none",
            x,
            "mkpart",
            "primary",
            "ext4",
            "0",
            constants::RADULA_FILE_GLAUCUS_IMAGE_SIZE,
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_PARTED)
        .args(&[
            constants::RADULA_TOOTH_PARTED_FLAGS,
            "-a",
            "none",
            x,
            "set",
            "1",
            "boot",
            "on",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new(constants::RADULA_TOOTH_MODPROBE)
        .arg("loop")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Find the first unused loop device
    let y = &String::from_utf8_lossy(
        &Command::new(constants::RADULA_TOOTH_LOSETUP)
            .arg("-f")
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .to_owned();

    let z = &[y, "p1"].concat();

    // Associate the first unused loop device with the image
    Command::new(constants::RADULA_TOOTH_LOSETUP)
        .arg("-D")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_LOSETUP)
        .args(&[y, x])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Notify the kernel about the new partition on the image
    Command::new(constants::RADULA_TOOTH_PARTX)
        .args(&["-a", y])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_MKE2FS)
        .args(&[constants::RADULA_TOOTH_MKE2FS_FLAGS, "ext4", z])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let w = &String::from(
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_PATH_MNT)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .to_str()
            .unwrap(),
    );

    fs::create_dir(w);

    Command::new(constants::RADULA_TOOTH_MOUNT)
        .args(&[z, w])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Remove `/lost+found` directory
    fs::remove_dir_all(Path::new(w).join(constants::RADULA_PATH_LOST_FOUND));

    radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap())
            .join(
                constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR
                    .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                    .unwrap(),
            )
            .to_str()
            .unwrap(),
        w,
    );

    let r = &String::from(
        Path::new(w)
            .join(constants::RADULA_PATH_BOOT)
            .join(constants::RADULA_TOOTH_EXTLINUX)
            .to_str()
            .unwrap(),
    );

    // Install extlinux as the default bootloader
    fs::create_dir(r);
    radula_behave_rsync(
        Path::new(constants::RADULA_PATH_CLUSTERS)
            .join(constants::RADULA_FILE_SYSLINUX_EXTLINUX_CONF)
            .to_str()
            .unwrap(),
        r,
    );
    Command::new(constants::RADULA_TOOTH_EXTLINUX)
        .args(&[constants::RADULA_TOOTH_EXTLINUX_FLAGS, r])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Change ownerships
    Command::new(constants::RADULA_TOOTH_CHOWN)
        .args(&[constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS, "0:0", w])
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_CHOWN)
        .args(&[
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
            "20:20",
            Path::new(w)
                .join(constants::RADULA_PATH_ETC)
                .join(constants::RADULA_PATH_UTMPS)
                .to_str()
                .unwrap(),
        ])
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Clean up
    Command::new(constants::RADULA_TOOTH_UMOUNT)
        .args(&[constants::RADULA_TOOTH_UMOUNT_FLAGS, w])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_PARTX)
        .args(&["-d", z])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_LOSETUP)
        .args(&["-d", z])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Backup the new image
    radula_behave_rsync(
        x,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap(),
    );
}
