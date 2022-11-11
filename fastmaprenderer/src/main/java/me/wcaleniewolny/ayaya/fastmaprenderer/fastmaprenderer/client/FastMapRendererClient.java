package me.wcaleniewolny.ayaya.fastmaprenderer.fastmaprenderer.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;

@Environment(EnvType.CLIENT)
public class FastMapRendererClient implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        System.out.println("NETTY " + System.getProperty("io.netty.eventLoopThreads"));
        System.setProperty("io.netty.eventLoopThreads", "24");
        System.out.println("NETTY " + System.getProperty("io.netty.eventLoopThreads"));
    }
}
