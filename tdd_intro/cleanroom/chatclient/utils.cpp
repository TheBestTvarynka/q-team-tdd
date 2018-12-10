#include "utils.h"

namespace
{
    std::string ReadAndValidateHandshake(ISocketWrapper& socket)
    {
        std::string data;
        ReadFromSocket(socket, data);

        auto separator = data.find(':');
        std::string serverMagic = data.substr(separator);

        if (serverMagic != ":HELLO!")
        {
            throw std::runtime_error("bad handshake");
        }
        return data.substr(0, separator);
    }
}

bool utils::TryToBind(ISocketWrapper& socket)
{
    try
    {
         socket.Bind("", 0);
    }
    catch (const std::exception& ex)
    {
        return false;
    }

    return true;
}

ISocketWrapperPtr utils::EstablishConnection(ISocketWrapper& socket)
{
    if (TryToBind(socket))
    {
        socket.Listen();
        return socket.Accept();
    }
    else
    {
        return socket.Connect("", 0);
    }
}

void utils::WriteToSocket(ISocketWrapper& socket, std::string data)
{
    data.push_back('\0');
    socket.Write(data);
}

void utils::ReadFromSocket(ISocketWrapper& socket, std::string& data)
{
    socket.Read(data);
}

std::string utils::ClientHandshake(ISocketWrapper& socket, const std::string& nickname)
{
    socket.Write(nickname + ":HELLO!");
    return ReadAndValidateHandshake(socket);
}

std::string utils::ServerHandshake(ISocketWrapper& socket, const std::string& nickname)
{
    std::string clientNickname = ReadAndValidateHandshake(socket);
    socket.Write(nickname + ":HELLO!");
    return clientNickname;
}