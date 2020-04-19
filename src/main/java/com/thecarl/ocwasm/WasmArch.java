package com.thecarl.ocwasm;

import li.cil.oc.api.machine.Architecture;
import li.cil.oc.api.machine.ExecutionResult;
import li.cil.oc.api.machine.Machine;
import net.minecraft.item.ItemStack;
import net.minecraft.nbt.NBTTagCompound;

import java.io.File;
import java.io.IOException;

import cz.adamh.utils.NativeUtils;

@Architecture.Name("WASM")
public class WasmArch implements Architecture {
    private final Machine machine;
    private long wasmPtr;

    static {
        // Load the shared library for our WASM interpreter.

        String osName = System.getProperty("os.name");

        String libName;

        if (osName.contains("Windows")) {
            // We are a Windows OS
            libName = "libwasm_interpreter.dll";
        } else if (osName.contains("Mac")) {
            // We are an Apple OS
            libName = "libwasm_interpreter.dylib";
        } else {
            // I dunno. Assume we're Linux.
            libName = "libwasm_interpreter.so";
        }

        try {
            NativeUtils.loadLibraryFromJar("/lib/" + libName);
        } catch (IOException e) {
            e.printStackTrace();

            // Since we failed to extract the jar file, make an attempt to load it from the filesystem.

            final File runningDir = new File(System.getProperty("user.dir"));
            System.load(runningDir + "/natives/" + libName);

            // If this fails too, then we will completely fail.
        }
    }

    private static native String hello();

    public WasmArch(Machine machine) {
        this.machine = machine;
      }

    @Override
    public void close() {
        // TODO Auto-generated method stub

    }

    @Override
    public boolean initialize() {
        System.out.println(hello());
        return true;
    }

    @Override
    public boolean isInitialized() {
        // TODO Auto-generated method stub
        return true;
    }

    @Override
    public void load(NBTTagCompound arg0) {
        // TODO Auto-generated method stub

    }

    @Override
    public void onConnect() {
        // TODO Auto-generated method stub

    }

    @Override
    public void onSignal() {
        // TODO Auto-generated method stub

    }

    @Override
    public boolean recomputeMemory(Iterable<ItemStack> arg0) {

        // The list you are given will tell you what components are in the machine.
        // Compute how much memory you have.
        // Return true if you have ANY memory.

        return true;
    }

    @Override
    public void runSynchronized() {
        // The interpreter will never interact with the world directly, so this will never be used.
    }

    @Override
    public ExecutionResult runThreaded(boolean arg0) {
        // TODO Auto-generated method stub
        return new ExecutionResult.Sleep(0);
    }

    @Override
    public void save(NBTTagCompound arg0) {
        // TODO Auto-generated method stub

    }

}