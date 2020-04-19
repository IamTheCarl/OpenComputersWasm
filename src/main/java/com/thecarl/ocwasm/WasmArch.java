package com.thecarl.ocwasm;

import li.cil.oc.api.machine.Architecture;
import li.cil.oc.api.machine.ExecutionResult;
import li.cil.oc.api.machine.Machine;
import net.minecraft.item.ItemStack;
import net.minecraft.nbt.NBTTagCompound;

import java.io.IOException;
import java.util.Vector;

import cz.adamh.utils.NativeUtils;

@Architecture.Name("WASM")
public class WasmArch implements Architecture {
    private final Machine machine;

    static {
        // Load the shared library for our WASM interpreter.
        try {
            // FIXME change the file extension based on OS.
            NativeUtils.loadLibraryFromJar("/lib/libwasm-interpreter.so");
        } catch (IOException e) {
            e.printStackTrace();
        }

        try {
            final java.lang.reflect.Field LIBRARIES = ClassLoader.class.getDeclaredField("loadedLibraryNames");
            LIBRARIES.setAccessible(true);
    
            final Vector<String> libraries = (Vector<String>) LIBRARIES.get(ClassLoader.getSystemClassLoader());
            for (String libName : libraries) {
                System.out.println(libName);
            }
        } catch (NoSuchFieldException | SecurityException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        } catch (IllegalArgumentException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        } catch (IllegalAccessException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
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
        return null;
    }

    @Override
    public void save(NBTTagCompound arg0) {
        // TODO Auto-generated method stub

    }

}