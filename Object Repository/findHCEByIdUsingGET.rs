<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>findHCEByIdUsingGET</name>
   <tag></tag>
   <elementGuidId>087d5e85-dd37-43d8-a434-db06a8f67982</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://localhost:9098/clevehr-services/api/v1/patients/${patientId}${/familyHistory/}${familyHistoryId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fd01b09b-101a-476a-9eb5-c1aa92fe8a59</id>
      <masked>false</masked>
      <name>familyHistoryId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>de38664f-e4f0-479f-91e6-7c3b8a4cecc0</id>
      <masked>false</masked>
      <name>patientId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>