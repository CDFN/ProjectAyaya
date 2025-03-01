package me.wcaleniewolny.ayaya.minecraft

import co.aikar.commands.PaperCommandManager
import me.wcaleniewolny.ayaya.minecraft.command.VideoCommand
import me.wcaleniewolny.ayaya.minecraft.command.VideoCommandCompletion
import me.wcaleniewolny.ayaya.minecraft.screen.ScreenController
import net.kyori.adventure.text.minimessage.MiniMessage
import org.bukkit.Bukkit
import org.bukkit.command.CommandSender
import org.bukkit.plugin.java.JavaPlugin
import java.io.BufferedReader
import java.io.File
import java.io.InputStream
import java.io.InputStreamReader
import java.math.BigInteger
import java.security.MessageDigest
import java.util.logging.Level


class MapMinecraftClient : JavaPlugin() {

    override fun onEnable() {
        this.saveDefaultConfig()

        if(!loadNativeLib()){
            return
        }

        val screenController = ScreenController(this);
        screenController.init()

        val manager = PaperCommandManager(this)
        val videoCommandCompletion = VideoCommandCompletion(screenController)

        videoCommandCompletion.init(this, manager)
        manager.registerCommand(
            VideoCommand(
                screenController,
                this.config,
                this
            )
        )

    }

    fun loadNativeLib(): Boolean{

        val unsafe = System.getProperty("me.wcaleniewolny.ayaya.unsafe") != null

        if(unsafe){
            logger.log(Level.WARNING, "UNSAFE LIB LOADING ENABLED")
            try{
                System.loadLibrary("ayaya_native")
            }catch (exception: UnsatisfiedLinkError){
                logger.log(Level.SEVERE, "Unable to load native library! AyayaNative will now get disabled")
                Bukkit.getPluginManager().disablePlugin(this)
                return false
            }
        } else {
            try {
                NativeUtils.loadLibraryFromJar("/libayaya_native.so")
            }catch (exception: Exception){
                logger.log(Level.SEVERE, "Unable to load native library! AyayaNative will now get disabled")
                Bukkit.getPluginManager().disablePlugin(this)
                exception.printStackTrace()
                return false
            }
        }

        return true
    }

    //https://mkyong.com/java/java-read-a-file-from-resources-folder/
    private fun getFileFromResourceAsStream(fileName: String): InputStream {

        // The class loader that loaded the class
        val classLoader: ClassLoader = this::class.java.classLoader
        val inputStream = classLoader.getResourceAsStream(fileName)

        // the stream holding the file content
        return inputStream ?: throw IllegalArgumentException("file not found! $fileName")
    }
}

fun CommandSender.sendColoredMessage(msg: String) {
    sendMessage(MiniMessage.miniMessage().deserialize(msg))
}